// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 Dana Schlifka

//! SQLite access: path resolution, schema, seeding and typed queries.
//!
//! The server is read-only and low-traffic, so a fresh connection is opened
//! per call. This keeps state minimal and avoids connection-pool management.

use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use rusqlite::Connection;

use crate::models::{Facets, Pattern, PatternType, SearchFilters};

/// DDL for both tables. Idempotent (IF NOT EXISTS).
const SCHEMA_SQL: &str = "
CREATE TABLE IF NOT EXISTS forms (
    id             INTEGER PRIMARY KEY,
    name           TEXT NOT NULL UNIQUE,
    description    TEXT NOT NULL DEFAULT '',
    focus          TEXT NOT NULL DEFAULT '',
    category       TEXT NOT NULL DEFAULT '',
    classification TEXT NOT NULL DEFAULT '',
    feature        TEXT NOT NULL DEFAULT '',
    tags           TEXT NOT NULL DEFAULT '[]',
    themes         TEXT NOT NULL DEFAULT '[]'
);
CREATE TABLE IF NOT EXISTS languages (
    id             INTEGER PRIMARY KEY,
    name           TEXT NOT NULL UNIQUE,
    description    TEXT NOT NULL DEFAULT '',
    focus          TEXT NOT NULL DEFAULT '',
    category       TEXT NOT NULL DEFAULT '',
    classification TEXT NOT NULL DEFAULT '',
    feature        TEXT NOT NULL DEFAULT '',
    tags           TEXT NOT NULL DEFAULT '[]',
    themes         TEXT NOT NULL DEFAULT '[]'
);
";

/// Example data, embedded into the binary. Only applied when empty.
const SEED_SQL: &str = include_str!("seed.sql");

/// Resolves the database path. `LAT_DB_PATH` takes precedence, otherwise the
/// platform data directory. Independent of the working directory, so the
/// server can be registered centrally (across projects).
pub fn db_path() -> Result<PathBuf> {
    if let Ok(p) = std::env::var("LAT_DB_PATH") {
        let path = PathBuf::from(p);
        // Create the parent directory if given and missing, otherwise
        // Connection::open fails inside a nonexistent directory.
        if let Some(parent) = path.parent()
            && !parent.as_os_str().is_empty()
        {
            std::fs::create_dir_all(parent)
                .with_context(|| format!("could not create directory {}", parent.display()))?;
        }
        return Ok(path);
    }
    let dir = dirs::data_dir()
        .context("no platform data directory found")?
        .join("lat");
    std::fs::create_dir_all(&dir)
        .with_context(|| format!("could not create directory {}", dir.display()))?;
    Ok(dir.join("patterns.db"))
}

/// Opens a connection to the database file.
pub fn open(path: &Path) -> Result<Connection> {
    Connection::open(path).with_context(|| format!("could not open database {}", path.display()))
}

/// Creates the schema and applies the seed when both tables are empty.
/// Returns `true` when the seed was applied by this call.
pub fn prepare(conn: &mut Connection) -> Result<bool> {
    conn.execute_batch(SCHEMA_SQL)
        .context("could not create schema")?;

    let count: i64 = conn.query_row(
        "SELECT (SELECT COUNT(*) FROM forms) + (SELECT COUNT(*) FROM languages)",
        [],
        |row| row.get(0),
    )?;
    if count != 0 {
        return Ok(false);
    }

    // Seed atomically: if seeding aborts midway, everything is rolled back
    // so no half-filled database is left behind that would wrongly count as
    // "already seeded" (count != 0) on the next start.
    let tx = conn.transaction()?;
    tx.execute_batch(SEED_SQL)
        .context("could not insert example data")?;
    tx.commit()?;
    Ok(true)
}

/// Creates the schema and seeds the database on first start.
/// Returns the resolved path.
pub fn init() -> Result<PathBuf> {
    let path = db_path()?;
    let mut conn = open(&path)?;
    if prepare(&mut conn)? {
        tracing::info!("database seeded: {}", path.display());
    }
    Ok(path)
}

/// Parses a JSON array from a TEXT field; empty on error.
fn parse_json_array(raw: &str) -> Vec<String> {
    serde_json::from_str(raw).unwrap_or_default()
}

/// Reads patterns from one table with optional filters.
fn query_table(
    conn: &Connection,
    kind: PatternType,
    filters: &SearchFilters,
) -> Result<Vec<Pattern>> {
    let table = kind.table();
    let mut sql = format!(
        "SELECT name, description, focus, category, classification, feature, tags, themes \
         FROM {table}"
    );
    let mut clauses: Vec<String> = Vec::new();
    let mut params: Vec<Box<dyn rusqlite::types::ToSql>> = Vec::new();

    if let Some(category) = &filters.category {
        clauses.push("category = ?".to_owned());
        params.push(Box::new(category.clone()));
    }
    if let Some(classification) = &filters.classification {
        clauses.push("classification = ?".to_owned());
        params.push(Box::new(classification.clone()));
    }
    if let Some(focus) = &filters.focus {
        clauses.push("focus LIKE ?".to_owned());
        params.push(Box::new(format!("%{focus}%")));
    }
    if let Some(text) = &filters.text {
        // tags are stored as a JSON array string, so a LIKE over the raw cell
        // also reaches keywords that never occur in the prose columns — about
        // two thirds of them (e.g. "body-part-locative", "coreference"). The
        // same argument covers classification: it holds the family, region and
        // typology vocabulary ("Slavic", "Bantu", "Australia", "isolate") and
        // its own filter matches the exact full string only, so without it that
        // vocabulary is advertised by list_facets yet unreachable by search.
        const TEXT_COLUMNS: [&str; 5] =
            ["name", "description", "feature", "tags", "classification"];
        let disjunction = TEXT_COLUMNS
            .iter()
            .map(|column| format!("{column} LIKE ?"))
            .collect::<Vec<_>>()
            .join(" OR ");
        clauses.push(format!("({disjunction})"));
        let like = format!("%{text}%");
        for _ in TEXT_COLUMNS {
            params.push(Box::new(like.clone()));
        }
    }
    if !filters.exclude_names.is_empty() {
        let placeholders = vec!["?"; filters.exclude_names.len()].join(", ");
        clauses.push(format!("name NOT IN ({placeholders})"));
        for name in &filters.exclude_names {
            params.push(Box::new(name.clone()));
        }
    }
    // json_each raises an error on invalid JSON that would abort the whole
    // query. The json_valid guard treats broken/empty cells as '[]' so one
    // faulty row cannot topple the search (relevant for hand-maintained
    // seed.sql and live edits).
    if let Some(tag) = &filters.tag {
        clauses.push(
            "EXISTS (SELECT 1 FROM json_each(\
             CASE WHEN json_valid(tags) THEN tags ELSE '[]' END) WHERE value = ?)"
                .to_owned(),
        );
        params.push(Box::new(tag.clone()));
    }
    if let Some(theme) = &filters.theme {
        clauses.push(
            "EXISTS (SELECT 1 FROM json_each(\
             CASE WHEN json_valid(themes) THEN themes ELSE '[]' END) WHERE value = ?)"
                .to_owned(),
        );
        params.push(Box::new(theme.clone()));
    }

    if !clauses.is_empty() {
        sql.push_str(" WHERE ");
        sql.push_str(&clauses.join(" AND "));
    }
    sql.push_str(" ORDER BY name");

    let mut stmt = conn.prepare(&sql)?;
    let param_refs: Vec<&dyn rusqlite::types::ToSql> =
        params.iter().map(|boxed| boxed.as_ref()).collect();
    let rows = stmt.query_map(param_refs.as_slice(), |row| {
        Ok(Pattern {
            kind,
            name: row.get(0)?,
            description: row.get(1)?,
            focus: row.get(2)?,
            category: row.get(3)?,
            classification: row.get(4)?,
            feature: row.get(5)?,
            tags: parse_json_array(&row.get::<_, String>(6)?),
            themes: parse_json_array(&row.get::<_, String>(7)?),
        })
    })?;

    let mut out = Vec::new();
    for row in rows {
        out.push(row?);
    }
    Ok(out)
}

/// Searches patterns across one or both kinds.
pub fn search(
    conn: &Connection,
    kind: Option<PatternType>,
    filters: &SearchFilters,
) -> Result<Vec<Pattern>> {
    let mut out = Vec::new();
    match kind {
        Some(k) => out.extend(query_table(conn, k, filters)?),
        None => {
            out.extend(query_table(conn, PatternType::Form, filters)?);
            out.extend(query_table(conn, PatternType::Language, filters)?);
        }
    }
    Ok(out)
}

/// Fetches a single pattern by kind and exact name.
pub fn get(conn: &Connection, kind: PatternType, name: &str) -> Result<Option<Pattern>> {
    let table = kind.table();
    let sql = format!(
        "SELECT name, description, focus, category, classification, feature, tags, themes \
         FROM {table} WHERE name = ?"
    );
    let mut stmt = conn.prepare(&sql)?;
    let mut rows = stmt.query_map([name], |row| {
        Ok(Pattern {
            kind,
            name: row.get(0)?,
            description: row.get(1)?,
            focus: row.get(2)?,
            category: row.get(3)?,
            classification: row.get(4)?,
            feature: row.get(5)?,
            tags: parse_json_array(&row.get::<_, String>(6)?),
            themes: parse_json_array(&row.get::<_, String>(7)?),
        })
    })?;
    match rows.next() {
        Some(row) => Ok(Some(row?)),
        None => Ok(None),
    }
}

/// Determines distinct values of a simple column.
fn distinct_column(conn: &Connection, table: &str, column: &str) -> Result<Vec<String>> {
    let sql =
        format!("SELECT DISTINCT {column} FROM {table} WHERE {column} <> '' ORDER BY {column}");
    let mut stmt = conn.prepare(&sql)?;
    let rows = stmt.query_map([], |row| row.get::<_, String>(0))?;
    let mut out = Vec::new();
    for row in rows {
        out.push(row?);
    }
    Ok(out)
}

/// Determines distinct values from a JSON-array field. The json_valid guard
/// keeps a broken cell from aborting the whole statement.
fn distinct_json(conn: &Connection, table: &str, column: &str) -> Result<Vec<String>> {
    let sql = format!(
        "SELECT DISTINCT value FROM {table}, \
         json_each(CASE WHEN json_valid({table}.{column}) THEN {table}.{column} ELSE '[]' END) \
         ORDER BY value"
    );
    let mut stmt = conn.prepare(&sql)?;
    let rows = stmt.query_map([], |row| row.get::<_, String>(0))?;
    let mut out = Vec::new();
    for row in rows {
        out.push(row?);
    }
    Ok(out)
}

/// Collects the available filter values of a table.
fn facets_for(conn: &Connection, kind: PatternType) -> Result<Facets> {
    let table = kind.table();
    Ok(Facets {
        kind,
        categories: distinct_column(conn, table, "category")?,
        classifications: distinct_column(conn, table, "classification")?,
        tags: distinct_json(conn, table, "tags")?,
        themes: distinct_json(conn, table, "themes")?,
    })
}

/// Returns facets for one or both kinds.
pub fn facets(conn: &Connection, kind: Option<PatternType>) -> Result<Vec<Facets>> {
    match kind {
        Some(k) => Ok(vec![facets_for(conn, k)?]),
        None => Ok(vec![
            facets_for(conn, PatternType::Form)?,
            facets_for(conn, PatternType::Language)?,
        ]),
    }
}

#[cfg(test)]
pub(crate) mod testing {
    //! Helpers shared by the unit tests of the other modules.

    use std::sync::atomic::{AtomicU32, Ordering};

    use super::*;

    static COUNTER: AtomicU32 = AtomicU32::new(0);

    /// A seeded database file in its own temporary directory, removed on drop.
    /// Tests need a real file rather than an in-memory database because the
    /// server opens a fresh connection per call from a path.
    pub(crate) struct TempDb {
        dir: PathBuf,
        path: PathBuf,
    }

    impl TempDb {
        pub(crate) fn new() -> Self {
            let unique = COUNTER.fetch_add(1, Ordering::Relaxed);
            let dir =
                std::env::temp_dir().join(format!("lat-test-{}-{unique}", std::process::id()));
            std::fs::create_dir_all(&dir).expect("could not create the temporary directory");
            let path = dir.join("patterns.db");
            let mut conn = open(&path).expect("could not open the temporary database");
            prepare(&mut conn).expect("could not prepare the temporary database");
            Self { dir, path }
        }

        pub(crate) fn path(&self) -> &Path {
            &self.path
        }
    }

    impl Drop for TempDb {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.dir);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Three languages and one form with known values, so the assertions stay
    /// stable when the catalogue in `seed.sql` grows. `Gamma` deliberately
    /// carries an invalid `tags` cell to exercise the `json_valid` guard.
    const FIXTURE_SQL: &str = r#"
INSERT INTO languages
    (name, description, focus, category, classification, feature, tags, themes)
VALUES
    ('Alpha', 'first sample', 'causal chain', 'Language', 'isolate',
     'marks the agent', '["alpha", "shared"]', '["Causality"]'),
    ('Beta', 'second sample', 'spatial frame', 'Register', 'Bantu',
     'marks the place', '["beta", "shared"]',
     '["Causality", "Space & orientation"]'),
    ('Gamma', 'third sample', '', 'Language', '',
     '', 'not json at all', '["Time & aspect"]');
INSERT INTO forms
    (name, description, focus, category, classification, feature, tags, themes)
VALUES
    ('Haiku', 'a cut between two images', 'brevity', 'Poetic form', 'Japanese',
     'seventeen morae', '["cut", "shared"]', '["Time & aspect"]');
"#;

    /// An in-memory database holding [`FIXTURE_SQL`], without the real seed.
    fn fixture() -> Connection {
        let conn = Connection::open_in_memory().expect("could not open an in-memory database");
        conn.execute_batch(SCHEMA_SQL).expect("schema failed");
        conn.execute_batch(FIXTURE_SQL).expect("fixture failed");
        conn
    }

    /// An in-memory database holding the real `seed.sql`.
    fn seeded() -> Connection {
        let mut conn = Connection::open_in_memory().expect("could not open an in-memory database");
        assert!(
            prepare(&mut conn).expect("prepare failed"),
            "expected a seed"
        );
        conn
    }

    fn names(patterns: &[Pattern]) -> Vec<&str> {
        patterns.iter().map(|p| p.name.as_str()).collect()
    }

    fn filters() -> SearchFilters {
        SearchFilters::default()
    }

    // ---- parse_json_array ------------------------------------------------

    #[test]
    fn parse_json_array_reads_a_string_array() {
        assert_eq!(parse_json_array(r#"["a", "b"]"#), vec!["a", "b"]);
    }

    #[test]
    fn parse_json_array_falls_back_to_empty_on_broken_input() {
        assert!(parse_json_array("not json").is_empty());
        assert!(parse_json_array("").is_empty());
        assert!(parse_json_array(r#"{"a": 1}"#).is_empty());
    }

    // ---- prepare ---------------------------------------------------------

    #[test]
    fn prepare_seeds_only_once() {
        let mut conn = Connection::open_in_memory().unwrap();
        assert!(prepare(&mut conn).unwrap(), "first call should seed");

        let before = search(&conn, None, &filters()).unwrap().len();
        assert!(before > 0, "the seed should not be empty");

        assert!(!prepare(&mut conn).unwrap(), "second call must not reseed");
        let after = search(&conn, None, &filters()).unwrap().len();
        assert_eq!(before, after, "reseeding would duplicate rows");
    }

    #[test]
    fn prepare_leaves_existing_rows_untouched() {
        let mut conn = Connection::open_in_memory().unwrap();
        conn.execute_batch(SCHEMA_SQL).unwrap();
        conn.execute_batch(FIXTURE_SQL).unwrap();

        assert!(
            !prepare(&mut conn).unwrap(),
            "a filled database is not empty"
        );
        let all = search(&conn, None, &filters()).unwrap();
        assert_eq!(names(&all), vec!["Haiku", "Alpha", "Beta", "Gamma"]);
    }

    // ---- search: shape ---------------------------------------------------

    #[test]
    fn search_without_filters_returns_forms_then_languages_each_sorted() {
        let conn = fixture();
        let all = search(&conn, None, &filters()).unwrap();
        assert_eq!(names(&all), vec!["Haiku", "Alpha", "Beta", "Gamma"]);
    }

    #[test]
    fn search_restricted_to_a_kind_reads_only_that_table() {
        let conn = fixture();

        let forms = search(&conn, Some(PatternType::Form), &filters()).unwrap();
        assert_eq!(names(&forms), vec!["Haiku"]);
        assert!(forms.iter().all(|p| p.kind == PatternType::Form));

        let languages = search(&conn, Some(PatternType::Language), &filters()).unwrap();
        assert_eq!(names(&languages), vec!["Alpha", "Beta", "Gamma"]);
        assert!(languages.iter().all(|p| p.kind == PatternType::Language));
    }

    #[test]
    fn every_column_is_mapped_onto_the_pattern() {
        let conn = fixture();
        let found = get(&conn, PatternType::Language, "Beta").unwrap().unwrap();

        assert_eq!(found.kind, PatternType::Language);
        assert_eq!(found.name, "Beta");
        assert_eq!(found.description, "second sample");
        assert_eq!(found.focus, "spatial frame");
        assert_eq!(found.category, "Register");
        assert_eq!(found.classification, "Bantu");
        assert_eq!(found.feature, "marks the place");
        assert_eq!(found.tags, vec!["beta", "shared"]);
        assert_eq!(found.themes, vec!["Causality", "Space & orientation"]);
    }

    // ---- search: individual filters --------------------------------------

    #[test]
    fn category_filter_matches_exactly() {
        let conn = fixture();
        let found = search(
            &conn,
            Some(PatternType::Language),
            &SearchFilters {
                category: Some("Register".to_owned()),
                ..filters()
            },
        )
        .unwrap();
        assert_eq!(names(&found), vec!["Beta"]);

        let partial = search(
            &conn,
            None,
            &SearchFilters {
                category: Some("Regis".to_owned()),
                ..filters()
            },
        )
        .unwrap();
        assert!(partial.is_empty(), "category is exact, not a substring");
    }

    #[test]
    fn classification_filter_matches_exactly() {
        let conn = fixture();
        let found = search(
            &conn,
            None,
            &SearchFilters {
                classification: Some("Japanese".to_owned()),
                ..filters()
            },
        )
        .unwrap();
        assert_eq!(names(&found), vec!["Haiku"]);
    }

    #[test]
    fn focus_filter_matches_a_substring() {
        let conn = fixture();
        let found = search(
            &conn,
            None,
            &SearchFilters {
                focus: Some("chain".to_owned()),
                ..filters()
            },
        )
        .unwrap();
        assert_eq!(names(&found), vec!["Alpha"]);
    }

    #[test]
    fn tag_filter_matches_a_whole_array_element() {
        let conn = fixture();
        let shared = search(
            &conn,
            None,
            &SearchFilters {
                tag: Some("shared".to_owned()),
                ..filters()
            },
        )
        .unwrap();
        assert_eq!(names(&shared), vec!["Haiku", "Alpha", "Beta"]);

        let partial = search(
            &conn,
            None,
            &SearchFilters {
                tag: Some("shar".to_owned()),
                ..filters()
            },
        )
        .unwrap();
        assert!(partial.is_empty(), "a tag matches the whole element only");
    }

    #[test]
    fn theme_filter_matches_a_whole_array_element() {
        let conn = fixture();
        let found = search(
            &conn,
            None,
            &SearchFilters {
                theme: Some("Space & orientation".to_owned()),
                ..filters()
            },
        )
        .unwrap();
        assert_eq!(names(&found), vec!["Beta"]);
    }

    #[test]
    fn text_filter_reaches_name_description_feature_tags_and_classification() {
        let conn = fixture();
        let by = |needle: &str| {
            let found = search(
                &conn,
                None,
                &SearchFilters {
                    text: Some(needle.to_owned()),
                    ..filters()
                },
            )
            .unwrap();
            names(&found)
                .iter()
                .map(|name| (*name).to_owned())
                .collect::<Vec<_>>()
        };

        assert_eq!(by("Haiku"), vec!["Haiku"], "name");
        assert_eq!(by("second sample"), vec!["Beta"], "description");
        assert_eq!(by("seventeen"), vec!["Haiku"], "feature");
        assert_eq!(by("alpha"), vec!["Alpha"], "tags");
        assert_eq!(by("Bantu"), vec!["Beta"], "classification");
    }

    #[test]
    fn text_filter_does_not_reach_the_focus_column() {
        // focus has its own filter; keeping it out of the free-text disjunction
        // is deliberate, so a change there shows up here.
        let conn = fixture();
        let found = search(
            &conn,
            None,
            &SearchFilters {
                text: Some("spatial frame".to_owned()),
                ..filters()
            },
        )
        .unwrap();
        assert!(found.is_empty());
    }

    #[test]
    fn exclude_names_drops_the_listed_patterns() {
        let conn = fixture();
        let found = search(
            &conn,
            None,
            &SearchFilters {
                exclude_names: vec!["Alpha".to_owned(), "Haiku".to_owned()],
                ..filters()
            },
        )
        .unwrap();
        assert_eq!(names(&found), vec!["Beta", "Gamma"]);
    }

    #[test]
    fn exclude_names_ignores_unknown_names() {
        let conn = fixture();
        let found = search(
            &conn,
            None,
            &SearchFilters {
                exclude_names: vec!["Nonexistent".to_owned()],
                ..filters()
            },
        )
        .unwrap();
        assert_eq!(found.len(), 4);
    }

    #[test]
    fn filters_are_combined_with_and() {
        let conn = fixture();
        let matching = search(
            &conn,
            None,
            &SearchFilters {
                tag: Some("shared".to_owned()),
                theme: Some("Causality".to_owned()),
                category: Some("Language".to_owned()),
                ..filters()
            },
        )
        .unwrap();
        assert_eq!(names(&matching), vec!["Alpha"]);

        let contradictory = search(
            &conn,
            None,
            &SearchFilters {
                tag: Some("alpha".to_owned()),
                theme: Some("Time & aspect".to_owned()),
                ..filters()
            },
        )
        .unwrap();
        assert!(contradictory.is_empty(), "AND, not OR");
    }

    // ---- search: robustness ----------------------------------------------

    #[test]
    fn a_row_with_invalid_json_tags_does_not_break_the_search() {
        let conn = fixture();

        // Unfiltered: the row is returned, with the broken cell read as empty.
        let all = search(&conn, Some(PatternType::Language), &filters()).unwrap();
        let gamma = all.iter().find(|p| p.name == "Gamma").unwrap();
        assert!(gamma.tags.is_empty());
        assert_eq!(gamma.themes, vec!["Time & aspect"]);

        // Filtered by tag: the query still succeeds and simply skips the row.
        let tagged = search(
            &conn,
            None,
            &SearchFilters {
                tag: Some("shared".to_owned()),
                ..filters()
            },
        )
        .unwrap();
        assert!(!names(&tagged).contains(&"Gamma"));
    }

    #[test]
    fn filter_values_are_bound_as_parameters_not_spliced_into_sql() {
        let conn = fixture();
        let injection = "' OR 1=1 --";

        for filters in [
            SearchFilters {
                text: Some(injection.to_owned()),
                ..filters()
            },
            SearchFilters {
                category: Some(injection.to_owned()),
                ..filters()
            },
            SearchFilters {
                tag: Some(injection.to_owned()),
                ..filters()
            },
            SearchFilters {
                exclude_names: vec![injection.to_owned()],
                category: Some("no such category".to_owned()),
                ..filters()
            },
        ] {
            let found = search(&conn, None, &filters).unwrap();
            assert!(found.is_empty(), "injected text must be data, not SQL");
        }
    }

    #[test]
    fn like_wildcards_in_a_text_filter_stay_wildcards() {
        // Documents current behaviour: the value goes into a LIKE pattern, so
        // '%' widens the match rather than being escaped.
        let conn = fixture();
        let found = search(
            &conn,
            None,
            &SearchFilters {
                text: Some("%".to_owned()),
                ..filters()
            },
        )
        .unwrap();
        assert_eq!(found.len(), 4);
    }

    // ---- get -------------------------------------------------------------

    #[test]
    fn get_returns_the_pattern_of_the_requested_kind() {
        let conn = fixture();
        let found = get(&conn, PatternType::Form, "Haiku").unwrap();
        assert_eq!(found.map(|p| p.name), Some("Haiku".to_owned()));
    }

    #[test]
    fn get_is_none_for_an_unknown_name() {
        let conn = fixture();
        assert!(get(&conn, PatternType::Form, "Sonnet").unwrap().is_none());
    }

    #[test]
    fn get_does_not_look_in_the_other_table() {
        let conn = fixture();
        assert!(
            get(&conn, PatternType::Language, "Haiku")
                .unwrap()
                .is_none(),
            "Haiku is a form, not a language"
        );
    }

    #[test]
    fn get_matches_the_name_exactly() {
        let conn = fixture();
        assert!(get(&conn, PatternType::Form, "Haik").unwrap().is_none());
        assert!(get(&conn, PatternType::Form, "haiku").unwrap().is_none());
    }

    // ---- facets ----------------------------------------------------------

    #[test]
    fn facets_for_one_kind_are_distinct_sorted_and_without_empties() {
        let conn = fixture();
        let facets = facets(&conn, Some(PatternType::Language)).unwrap();
        assert_eq!(facets.len(), 1);

        let languages = &facets[0];
        assert_eq!(languages.kind, PatternType::Language);
        assert_eq!(languages.categories, vec!["Language", "Register"]);
        assert_eq!(languages.classifications, vec!["Bantu", "isolate"]);
        assert_eq!(languages.tags, vec!["alpha", "beta", "shared"]);
        assert_eq!(
            languages.themes,
            vec!["Causality", "Space & orientation", "Time & aspect"]
        );
    }

    #[test]
    fn facets_without_a_kind_return_forms_then_languages() {
        let conn = fixture();
        let facets = facets(&conn, None).unwrap();
        assert_eq!(facets.len(), 2);
        assert_eq!(facets[0].kind, PatternType::Form);
        assert_eq!(facets[1].kind, PatternType::Language);
        assert_eq!(facets[0].tags, vec!["cut", "shared"]);
    }

    #[test]
    fn facets_survive_a_row_with_invalid_json() {
        // Gamma's tags cell is broken; the guard must keep the statement alive
        // and simply contribute nothing.
        let conn = fixture();
        let facets = facets(&conn, Some(PatternType::Language)).unwrap();
        assert_eq!(facets[0].tags, vec!["alpha", "beta", "shared"]);
    }

    // ---- the shipped catalogue -------------------------------------------

    #[test]
    fn the_seed_fills_both_tables() {
        let conn = seeded();
        assert!(
            !search(&conn, Some(PatternType::Form), &filters())
                .unwrap()
                .is_empty()
        );
        assert!(
            !search(&conn, Some(PatternType::Language), &filters())
                .unwrap()
                .is_empty()
        );
    }

    #[test]
    fn every_seeded_row_carries_valid_json_arrays() {
        let conn = seeded();
        for pattern in search(&conn, None, &filters()).unwrap() {
            let sql = format!(
                "SELECT tags, themes FROM {} WHERE name = ?",
                pattern.kind.table()
            );
            let (tags, themes): (String, String) = conn
                .query_row(&sql, [&pattern.name], |row| Ok((row.get(0)?, row.get(1)?)))
                .unwrap();
            assert!(
                serde_json::from_str::<Vec<String>>(&tags).is_ok(),
                "{}: tags is not a JSON string array: {tags}",
                pattern.name
            );
            assert!(
                serde_json::from_str::<Vec<String>>(&themes).is_ok(),
                "{}: themes is not a JSON string array: {themes}",
                pattern.name
            );
        }
    }

    #[test]
    fn every_seeded_facet_value_is_reachable_by_a_filter() {
        // Guards the promise of list_facets: whatever it advertises must yield
        // at least one hit when handed back to search_patterns.
        let conn = seeded();
        for facet in facets(&conn, None).unwrap() {
            let kind = Some(facet.kind);
            let expect_hit = |filters: SearchFilters, label: &str| {
                let found = search(&conn, kind, &filters).unwrap();
                assert!(!found.is_empty(), "{label} matches nothing");
            };

            for tag in &facet.tags {
                expect_hit(
                    SearchFilters {
                        tag: Some(tag.clone()),
                        ..filters()
                    },
                    &format!("tag '{tag}'"),
                );
            }
            for theme in &facet.themes {
                expect_hit(
                    SearchFilters {
                        theme: Some(theme.clone()),
                        ..filters()
                    },
                    &format!("theme '{theme}'"),
                );
            }
            for category in &facet.categories {
                expect_hit(
                    SearchFilters {
                        category: Some(category.clone()),
                        ..filters()
                    },
                    &format!("category '{category}'"),
                );
            }
            for classification in &facet.classifications {
                expect_hit(
                    SearchFilters {
                        classification: Some(classification.clone()),
                        ..filters()
                    },
                    &format!("classification '{classification}'"),
                );
            }
        }
    }

    #[test]
    fn the_seeded_baseline_language_is_present_and_excludable() {
        let conn = seeded();
        assert!(
            get(&conn, PatternType::Language, "German")
                .unwrap()
                .is_some()
        );

        let without = search(
            &conn,
            Some(PatternType::Language),
            &SearchFilters {
                exclude_names: vec!["German".to_owned()],
                ..filters()
            },
        )
        .unwrap();
        assert!(!names(&without).contains(&"German"));
    }
}
