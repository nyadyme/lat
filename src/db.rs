// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 Dana Schlifka

//! SQLite access: path resolution, schema, seeding and typed queries.
//!
//! The server is read-only and low-traffic, so a fresh connection is opened
//! per call. This keeps state minimal and avoids connection-pool management.

use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use rusqlite::{Connection, OptionalExtension};

use crate::models::{
    Combination, CombinationRoute, DroppedLens, Facets, Pattern, PatternType, RosterLens,
    RoutingOverview, RoutingProfile, SearchFilters,
};

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
    forced_choice  TEXT NOT NULL DEFAULT '',
    attachment     TEXT NOT NULL DEFAULT '',
    tags           TEXT NOT NULL DEFAULT '[]',
    themes         TEXT NOT NULL DEFAULT '[]',
    source         TEXT NOT NULL DEFAULT '',
    status         TEXT NOT NULL DEFAULT ''
);
CREATE TABLE IF NOT EXISTS languages (
    id             INTEGER PRIMARY KEY,
    name           TEXT NOT NULL UNIQUE,
    description    TEXT NOT NULL DEFAULT '',
    focus          TEXT NOT NULL DEFAULT '',
    category       TEXT NOT NULL DEFAULT '',
    classification TEXT NOT NULL DEFAULT '',
    feature        TEXT NOT NULL DEFAULT '',
    forced_choice  TEXT NOT NULL DEFAULT '',
    attachment     TEXT NOT NULL DEFAULT '',
    tags           TEXT NOT NULL DEFAULT '[]',
    themes         TEXT NOT NULL DEFAULT '[]',
    source         TEXT NOT NULL DEFAULT '',
    status         TEXT NOT NULL DEFAULT ''
);
";

/// Columns introduced after the first release, with their declaration.
/// `CREATE TABLE IF NOT EXISTS` leaves an existing table alone, so a database
/// written by an older build lacks them and every query would fail with
/// "no such column".
/// The default is what makes an added column possible at all — SQLite refuses
/// `ADD COLUMN ... NOT NULL` without one — and it is also the safe value here:
/// an empty `status` is not `sourced`, so a row carried over from an older
/// database is filtered out by `exclude_contested` rather than passed off as
/// checked.
const ADDED_COLUMNS: [(&str, &str); 4] = [
    ("forced_choice", "TEXT NOT NULL DEFAULT ''"),
    ("attachment", "TEXT NOT NULL DEFAULT ''"),
    ("source", "TEXT NOT NULL DEFAULT ''"),
    ("status", "TEXT NOT NULL DEFAULT ''"),
];

/// DDL for the combinations and their per-language routing. Kept apart from
/// [`SCHEMA_SQL`] because it is seeded apart: a database written before
/// routing existed has full pattern tables and empty routing tables, and must
/// still pick the routing up.
const ROUTING_SCHEMA_SQL: &str = "
CREATE TABLE IF NOT EXISTS combinations (
    id           TEXT PRIMARY KEY,
    name         TEXT NOT NULL DEFAULT '',
    axis         TEXT NOT NULL DEFAULT '',
    trigger_text TEXT NOT NULL DEFAULT '',
    provenance   TEXT NOT NULL DEFAULT ''
);
CREATE TABLE IF NOT EXISTS combination_lenses (
    combination TEXT NOT NULL,
    position    INTEGER NOT NULL,
    name        TEXT NOT NULL,
    kind        TEXT NOT NULL,
    role        TEXT NOT NULL DEFAULT '',
    polarity    TEXT NOT NULL DEFAULT '',
    PRIMARY KEY (combination, position)
);
CREATE TABLE IF NOT EXISTS routing_profiles (
    source_language TEXT PRIMARY KEY COLLATE NOCASE,
    own_entries     TEXT NOT NULL DEFAULT '[]',
    note            TEXT NOT NULL DEFAULT ''
);
CREATE TABLE IF NOT EXISTS routing (
    source_language TEXT NOT NULL COLLATE NOCASE,
    combination     TEXT NOT NULL,
    verdict         TEXT NOT NULL,
    marker          TEXT NOT NULL DEFAULT '',
    note            TEXT NOT NULL DEFAULT '',
    citation        TEXT NOT NULL DEFAULT '',
    provenance      TEXT NOT NULL DEFAULT '',
    PRIMARY KEY (source_language, combination)
);
CREATE TABLE IF NOT EXISTS substitutions (
    source_language TEXT NOT NULL COLLATE NOCASE,
    combination     TEXT NOT NULL,
    removed         TEXT NOT NULL,
    replacement     TEXT NOT NULL DEFAULT '',
    replacement_kind TEXT NOT NULL DEFAULT '',
    reason          TEXT NOT NULL DEFAULT '',
    PRIMARY KEY (source_language, combination, removed)
);
";

/// Example data, embedded into the binary. Only applied when empty.
const SEED_SQL: &str = include_str!("seed.sql");

/// Routing data, embedded into the binary. Applied when the routing tables
/// are empty, independently of [`SEED_SQL`].
const ROUTING_SEED_SQL: &str = include_str!("routing_seed.sql");

/// Adds any column of [`ADDED_COLUMNS`] the table does not have yet.
///
/// The column is added in place rather than by rebuilding the table: the
/// database is documented as live-editable, and a rebuild would silently throw
/// those edits away. The added cells stay empty until the file is deleted and
/// reseeded from the catalogue, which is what the warning says.
fn add_missing_columns(conn: &Connection) -> Result<()> {
    for table in [PatternType::Form.table(), PatternType::Language.table()] {
        // Both the table and the column names are compile-time constants, so
        // the formatted DDL carries no caller input.
        let mut stmt = conn.prepare(&format!("PRAGMA table_info({table})"))?;
        let present: Vec<String> = stmt
            .query_map([], |row| row.get::<_, String>(1))?
            .collect::<rusqlite::Result<_>>()?;
        for (column, declaration) in ADDED_COLUMNS {
            if present.iter().any(|have| have == column) {
                continue;
            }
            conn.execute_batch(&format!(
                "ALTER TABLE {table} ADD COLUMN {column} {declaration}"
            ))
            .with_context(|| format!("could not add {table}.{column}"))?;
            tracing::warn!(
                "{table}.{column} was missing and has been added empty; \
                 delete the database file and restart to fill it from the catalogue"
            );
        }
    }
    Ok(())
}

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

/// Which seeds a call to [`prepare`] applied.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Seeded {
    /// The pattern tables were empty and have been filled.
    pub patterns: bool,
    /// The routing tables were empty and have been filled.
    pub routing: bool,
}

/// Creates the schema and applies each seed whose tables are empty: the
/// patterns when both pattern tables are empty, the routing when the
/// combinations table is. The two are reported apart because an older
/// database has the first and lacks the second.
pub fn prepare(conn: &mut Connection) -> Result<Seeded> {
    conn.execute_batch(SCHEMA_SQL)
        .context("could not create schema")?;
    add_missing_columns(conn)?;
    conn.execute_batch(ROUTING_SCHEMA_SQL)
        .context("could not create the routing schema")?;

    let patterns = seed_if_empty(
        conn,
        "SELECT (SELECT COUNT(*) FROM forms) + (SELECT COUNT(*) FROM languages)",
        SEED_SQL,
        "could not insert example data",
    )?;
    let routing = seed_if_empty(
        conn,
        "SELECT COUNT(*) FROM combinations",
        ROUTING_SEED_SQL,
        "could not insert routing data",
    )?;
    Ok(Seeded { patterns, routing })
}

/// Applies `seed` when `count_sql` returns zero. Returns whether it did.
///
/// Seeds atomically: if seeding aborts midway, everything is rolled back so no
/// half-filled database is left behind that would wrongly count as "already
/// seeded" on the next start.
fn seed_if_empty(conn: &mut Connection, count_sql: &str, seed: &str, what: &str) -> Result<bool> {
    let count: i64 = conn.query_row(count_sql, [], |row| row.get(0))?;
    if count != 0 {
        return Ok(false);
    }
    let tx = conn.transaction()?;
    tx.execute_batch(seed).context(what.to_owned())?;
    tx.commit()?;
    Ok(true)
}

/// Creates the schema and seeds the database on first start.
/// Returns the resolved path.
pub fn init() -> Result<PathBuf> {
    let path = db_path()?;
    let mut conn = open(&path)?;
    let seeded = prepare(&mut conn)?;
    if seeded.patterns {
        tracing::info!("patterns seeded: {}", path.display());
    }
    if seeded.routing {
        tracing::info!("routing seeded: {}", path.display());
    }
    let missing = missing_lenses(&conn)?;
    if !missing.is_empty() {
        tracing::warn!(
            "the routing names lenses this database's catalogue lacks: {}; delete the \
             database file and restart to reseed the catalogue",
            missing.join(", ")
        );
    }
    Ok(path)
}

/// Parses a JSON array from a TEXT field; empty on error.
fn parse_json_array(raw: &str) -> Vec<String> {
    serde_json::from_str(raw).unwrap_or_default()
}

/// Bound parameters for a dynamically assembled statement.
type SqlParams = Vec<Box<dyn rusqlite::types::ToSql>>;

/// Translates the filters into WHERE clauses and their bound parameters, in
/// matching order. The clauses are AND-combined by the caller.
fn filter_clauses(filters: &SearchFilters) -> (Vec<String>, SqlParams) {
    let mut clauses: Vec<String> = Vec::new();
    let mut params: SqlParams = Vec::new();

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
    if let Some(forced_choice) = &filters.forced_choice {
        clauses.push("forced_choice LIKE ?".to_owned());
        params.push(Box::new(format!("%{forced_choice}%")));
    }
    if let Some(attachment) = &filters.attachment {
        clauses.push("attachment = ?".to_owned());
        params.push(Box::new(attachment.clone()));
    }
    if let Some(text) = &filters.text {
        // tags are stored as a JSON array string, so a LIKE over the raw cell
        // also reaches keywords that never occur in the prose columns — about
        // two thirds of them (e.g. "body-part-locative", "coreference"). The
        // same argument covers classification: it holds the family, region and
        // typology vocabulary ("Slavic", "Bantu", "Australia", "isolate") and
        // its own filter matches the exact full string only, so without it that
        // vocabulary is advertised by list_facets yet unreachable by search.
        const TEXT_COLUMNS: [&str; 6] = [
            "name",
            "description",
            "feature",
            "tags",
            "classification",
            "source",
        ];
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
    if filters.exclude_contested {
        // Tested as "is sourced" rather than "is not contested" on purpose: an
        // entry whose source has not been filled in yet carries neither value,
        // and letting it through would defeat the filter at exactly the rows
        // with the least backing behind them.
        clauses.push("status = 'sourced'".to_owned());
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
    (clauses, params)
}

/// Reads patterns from one table with optional filters.
fn query_table(
    conn: &Connection,
    kind: PatternType,
    filters: &SearchFilters,
) -> Result<Vec<Pattern>> {
    let table = kind.table();
    let mut sql = format!(
        "SELECT name, description, focus, category, classification, feature, forced_choice, \
         attachment, tags, themes, source, status \
         FROM {table}"
    );
    let (clauses, params) = filter_clauses(filters);
    if !clauses.is_empty() {
        sql.push_str(" WHERE ");
        sql.push_str(&clauses.join(" AND "));
    }
    sql.push_str(" ORDER BY name");

    let mut stmt = conn.prepare(&sql)?;
    let param_refs: Vec<&dyn rusqlite::types::ToSql> = params.iter().map(AsRef::as_ref).collect();
    let rows = stmt.query_map(param_refs.as_slice(), |row| {
        Ok(Pattern {
            kind,
            name: row.get(0)?,
            description: row.get(1)?,
            focus: row.get(2)?,
            category: row.get(3)?,
            classification: row.get(4)?,
            feature: row.get(5)?,
            forced_choice: row.get(6)?,
            attachment: row.get(7)?,
            tags: parse_json_array(&row.get::<_, String>(8)?),
            themes: parse_json_array(&row.get::<_, String>(9)?),
            source: row.get(10)?,
            status: row.get(11)?,
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
    if let Some(k) = kind {
        out.extend(query_table(conn, k, filters)?);
    } else {
        out.extend(query_table(conn, PatternType::Form, filters)?);
        out.extend(query_table(conn, PatternType::Language, filters)?);
    }
    Ok(out)
}

/// Fetches a single pattern by kind and exact name.
pub fn get(conn: &Connection, kind: PatternType, name: &str) -> Result<Option<Pattern>> {
    let table = kind.table();
    let sql = format!(
        "SELECT name, description, focus, category, classification, feature, forced_choice, \
         attachment, tags, themes, source, status \
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
            forced_choice: row.get(6)?,
            attachment: row.get(7)?,
            tags: parse_json_array(&row.get::<_, String>(8)?),
            themes: parse_json_array(&row.get::<_, String>(9)?),
            source: row.get(10)?,
            status: row.get(11)?,
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

/// Determines distinct values from a JSON-array field. The `json_valid` guard
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
        attachments: distinct_column(conn, table, "attachment")?,
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

/// One substitution row of a profile: the removed lens, its replacement
/// (empty when dropped) with its kind, and the reason.
///
/// The kind is stored with the row rather than looked up in the catalogue: a
/// database seeded by an older build keeps its old pattern tables, and a
/// replacement added to the catalogue since would otherwise make the whole
/// profile fail.
struct Substitution {
    removed: String,
    replacement: String,
    replacement_kind: String,
    reason: String,
}

/// Parses a stored kind. An unknown value means a corrupted row, and passing it
/// off as either kind would send the caller to the wrong table.
fn parse_kind(raw: &str) -> Result<PatternType> {
    match raw {
        "form" => Ok(PatternType::Form),
        "language" => Ok(PatternType::Language),
        other => anyhow::bail!("unknown pattern kind {other:?} in the routing tables"),
    }
}

/// Lens names the routing enlists — in a roster or as a replacement — that
/// the pattern tables do not hold, sorted.
///
/// Empty on a database seeded by the current build. A database seeded by an
/// older one keeps its old catalogue while picking up the routing, so a lens
/// added to the catalogue since can be routed to but not looked up.
pub fn missing_lenses(conn: &Connection) -> Result<Vec<String>> {
    let mut stmt = conn.prepare(
        "SELECT name FROM combination_lenses \
         UNION SELECT replacement FROM substitutions WHERE replacement <> '' \
         EXCEPT SELECT name FROM languages \
         EXCEPT SELECT name FROM forms \
         ORDER BY 1",
    )?;
    let rows = stmt.query_map([], |row| row.get::<_, String>(0))?;
    Ok(rows.collect::<rusqlite::Result<_>>()?)
}

/// Reads the language-neutral roster of one combination, in run order.
fn roster_of(conn: &Connection, combination: &str) -> Result<Vec<RosterLens>> {
    let mut stmt = conn.prepare(
        "SELECT position, name, kind, role, polarity FROM combination_lenses \
         WHERE combination = ? ORDER BY position",
    )?;
    let rows = stmt.query_map([combination], |row| {
        Ok((
            row.get::<_, u32>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, String>(2)?,
            row.get::<_, String>(3)?,
            row.get::<_, String>(4)?,
        ))
    })?;
    let mut out = Vec::new();
    for row in rows {
        let (position, name, kind, role, polarity) = row?;
        out.push(RosterLens {
            position,
            name,
            kind: parse_kind(&kind)?,
            role,
            polarity,
            replaces: None,
            reason: None,
        });
    }
    Ok(out)
}

/// Reads every combination with its language-neutral roster, in seed order.
pub fn combinations(conn: &Connection) -> Result<Vec<Combination>> {
    let mut stmt = conn.prepare(
        "SELECT id, name, axis, trigger_text, provenance FROM combinations ORDER BY rowid",
    )?;
    let heads = stmt
        .query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?,
                row.get::<_, String>(4)?,
            ))
        })?
        .collect::<rusqlite::Result<Vec<_>>>()?;
    let mut out = Vec::new();
    for (id, name, axis, trigger, provenance) in heads {
        let roster = roster_of(conn, &id)?;
        out.push(Combination {
            id,
            name,
            axis,
            trigger,
            provenance,
            roster,
        });
    }
    Ok(out)
}

/// Names of every routing profile, in seed order.
pub fn routing_profiles(conn: &Connection) -> Result<Vec<String>> {
    let mut stmt = conn.prepare("SELECT source_language FROM routing_profiles ORDER BY rowid")?;
    let rows = stmt.query_map([], |row| row.get::<_, String>(0))?;
    Ok(rows.collect::<rusqlite::Result<_>>()?)
}

/// Applies a profile's substitutions to one roster.
///
/// A replacement takes the slot of the lens it removes — position, role and
/// polarity stay, because the doctrine assigns those to the slot, not to the
/// lens. A substitution with an empty replacement drops the slot.
fn apply_substitutions(
    roster: Vec<RosterLens>,
    subs: &[Substitution],
) -> Result<(Vec<RosterLens>, Vec<DroppedLens>)> {
    let mut lenses = Vec::new();
    let mut dropped = Vec::new();
    for lens in roster {
        let Some(sub) = subs.iter().find(|s| s.removed == lens.name) else {
            lenses.push(lens);
            continue;
        };
        if sub.replacement.is_empty() {
            dropped.push(DroppedLens {
                name: lens.name,
                reason: sub.reason.clone(),
            });
            continue;
        }
        lenses.push(RosterLens {
            kind: parse_kind(&sub.replacement_kind)?,
            name: sub.replacement.clone(),
            replaces: Some(lens.name),
            reason: Some(sub.reason.clone()),
            ..lens
        });
    }
    Ok((lenses, dropped))
}

/// Reads the substitutions one profile makes in one combination.
fn substitutions_of(
    conn: &Connection,
    source: &str,
    combination: &str,
) -> Result<Vec<Substitution>> {
    let mut stmt = conn.prepare(
        "SELECT removed, replacement, replacement_kind, reason FROM substitutions \
         WHERE source_language = ? AND combination = ?",
    )?;
    let rows = stmt.query_map([source, combination], |row| {
        Ok(Substitution {
            removed: row.get(0)?,
            replacement: row.get(1)?,
            replacement_kind: row.get(2)?,
            reason: row.get(3)?,
        })
    })?;
    Ok(rows.collect::<rusqlite::Result<_>>()?)
}

/// Routes one combination for one profile.
fn route(conn: &Connection, source: &str, combination: Combination) -> Result<CombinationRoute> {
    let row = conn
        .query_row(
            "SELECT verdict, marker, note, citation, provenance FROM routing \
             WHERE source_language = ? AND combination = ?",
            [source, combination.id.as_str()],
            |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, String>(2)?,
                    row.get::<_, String>(3)?,
                    row.get::<_, String>(4)?,
                ))
            },
        )
        .optional()?;
    // The generator refuses an incomplete profile; a missing row can only come
    // from a live edit, and is reported rather than skipped so the combination
    // does not silently vanish from the caller's routing.
    let (verdict, marker, note, citation, provenance) = row.unwrap_or_else(|| {
        (
            "unrouted".to_owned(),
            String::new(),
            "the database holds no routing row for this combination".to_owned(),
            String::new(),
            String::new(),
        )
    });
    let subs = substitutions_of(conn, source, &combination.id)?;
    let (roster, dropped) = apply_substitutions(combination.roster, &subs)?;
    Ok(CombinationRoute {
        id: combination.id,
        name: combination.name,
        axis: combination.axis,
        trigger: combination.trigger,
        verdict,
        marker,
        note,
        citation,
        provenance,
        roster,
        dropped,
    })
}

/// Returns the routing profile of a source language, matched without regard
/// to case, or `None` when no profile exists for it.
pub fn routing(conn: &Connection, source_language: &str) -> Result<Option<RoutingProfile>> {
    let head = conn
        .query_row(
            "SELECT source_language, own_entries, note FROM routing_profiles \
             WHERE source_language = ?",
            [source_language.trim()],
            |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, String>(2)?,
                ))
            },
        )
        .optional()?;
    let Some((name, own_entries, note)) = head else {
        return Ok(None);
    };
    let mut routed = Vec::new();
    for combination in combinations(conn)? {
        routed.push(route(conn, &name, combination)?);
    }
    let missing = missing_lenses(conn)?;
    let missing_from_catalogue = missing
        .into_iter()
        .filter(|lens| {
            routed
                .iter()
                .any(|c| c.roster.iter().any(|r| &r.name == lens))
        })
        .collect();
    Ok(Some(RoutingProfile {
        source_language: name,
        own_entries: parse_json_array(&own_entries),
        note,
        missing_from_catalogue,
        combinations: routed,
    }))
}

/// The profile names and the language-neutral combinations, with an optional
/// message for the caller.
pub fn routing_overview(conn: &Connection, message: Option<String>) -> Result<RoutingOverview> {
    Ok(RoutingOverview {
        message,
        profiles: routing_profiles(conn)?,
        combinations: combinations(conn)?,
    })
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
#[allow(
    clippy::assert_is_empty,
    clippy::cast_possible_truncation,
    reason = "assert!(x.is_empty()) reads as the behavior under test; the fuzz casts are bounded by a modulo"
)]
mod tests {
    use super::*;

    /// Three languages and one form with known values, so the assertions stay
    /// stable when the catalogue in `seed.sql` grows. `Gamma` deliberately
    /// carries an invalid `tags` cell to exercise the `json_valid` guard, and
    /// shares its (`forced_choice`, `attachment`) pair with `Alpha` so a collision
    /// is present in the fixture.
    const FIXTURE_SQL: &str = r#"
INSERT INTO languages
    (name, description, focus, category, classification, feature,
     forced_choice, attachment, tags, themes, source, status)
VALUES
    ('Alpha', 'first sample', 'causal chain', 'Language', 'isolate',
     'marks the agent', 'whether the act was willed', 'subject',
     '["alpha", "shared"]', '["Causality"]', 'Ekaterina, A Grammar of Alpha', 'sourced'),
    ('Beta', 'second sample', 'spatial frame', 'Register', 'Bantu',
     'marks the place', 'where the thing stands', 'noun',
     '["beta", "shared"]', '["Causality", "Space & orientation"]',
     'Bhatt (1974); contra Oyelaran (1990)', 'contested'),
    ('Gamma', 'third sample', '', 'Language', '',
     '', 'whether the act was willed', 'subject',
     'not json at all', '["Time & aspect"]', '', '');
INSERT INTO forms
    (name, description, focus, category, classification, feature,
     forced_choice, attachment, tags, themes, source, status)
VALUES
    ('Haiku', 'a cut between two images', 'brevity', 'Poetic form', 'Japanese',
     'seventeen morae', 'whether two images need a connective',
     'whole passage', '["cut", "shared"]', '["Time & aspect"]',
     'Higginson, The Haiku Handbook', 'sourced');
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
        let seeded = prepare(&mut conn).expect("prepare failed");
        assert!(seeded.patterns && seeded.routing, "expected both seeds");
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
        assert_eq!(
            prepare(&mut conn).unwrap(),
            Seeded {
                patterns: true,
                routing: true
            },
            "first call should apply both seeds"
        );

        let before = search(&conn, None, &filters()).unwrap().len();
        assert!(before > 0, "the seed should not be empty");

        assert_eq!(
            prepare(&mut conn).unwrap(),
            Seeded {
                patterns: false,
                routing: false
            },
            "second call must not reseed"
        );
        let after = search(&conn, None, &filters()).unwrap().len();
        assert_eq!(before, after, "reseeding would duplicate rows");
    }

    #[test]
    fn prepare_adds_columns_missing_from_an_older_database() {
        // A database written before the combination columns existed: the
        // rows have to survive, and a query must not fail on the new columns.
        const OLD_SCHEMA: &str = "
CREATE TABLE forms (
    id INTEGER PRIMARY KEY, name TEXT NOT NULL UNIQUE,
    description TEXT NOT NULL DEFAULT '', focus TEXT NOT NULL DEFAULT '',
    category TEXT NOT NULL DEFAULT '', classification TEXT NOT NULL DEFAULT '',
    feature TEXT NOT NULL DEFAULT '', tags TEXT NOT NULL DEFAULT '[]',
    themes TEXT NOT NULL DEFAULT '[]'
);
CREATE TABLE languages (
    id INTEGER PRIMARY KEY, name TEXT NOT NULL UNIQUE,
    description TEXT NOT NULL DEFAULT '', focus TEXT NOT NULL DEFAULT '',
    category TEXT NOT NULL DEFAULT '', classification TEXT NOT NULL DEFAULT '',
    feature TEXT NOT NULL DEFAULT '', tags TEXT NOT NULL DEFAULT '[]',
    themes TEXT NOT NULL DEFAULT '[]'
);
INSERT INTO languages (name, focus) VALUES ('Hand-edited', 'kept');
";
        let mut conn = Connection::open_in_memory().expect("in-memory database");
        conn.execute_batch(OLD_SCHEMA).expect("old schema failed");

        let seeded = prepare(&mut conn).expect("prepare failed");
        assert!(
            !seeded.patterns,
            "a non-empty database must not be reseeded"
        );

        let found = search(&conn, None, &filters()).unwrap();
        assert_eq!(names(&found), vec!["Hand-edited"], "the live edit was lost");
        assert_eq!(found[0].forced_choice, "");
        assert_eq!(found[0].attachment, "");
    }

    #[test]
    fn prepare_leaves_existing_rows_untouched() {
        let mut conn = Connection::open_in_memory().unwrap();
        conn.execute_batch(SCHEMA_SQL).unwrap();
        conn.execute_batch(FIXTURE_SQL).unwrap();

        assert!(
            !prepare(&mut conn).unwrap().patterns,
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
        assert_eq!(found.forced_choice, "where the thing stands");
        assert_eq!(found.attachment, "noun");
        assert_eq!(found.tags, vec!["beta", "shared"]);
        assert_eq!(found.themes, vec!["Causality", "Space & orientation"]);
        assert_eq!(found.source, "Bhatt (1974); contra Oyelaran (1990)");
        assert_eq!(found.status, "contested");
    }

    // ---- search: individual filters --------------------------------------

    #[test]
    fn exclude_contested_keeps_only_what_carries_a_source_and_is_undisputed() {
        let conn = fixture();
        let found = search(
            &conn,
            Some(PatternType::Language),
            &SearchFilters {
                exclude_contested: true,
                ..SearchFilters::default()
            },
        )
        .unwrap();
        assert_eq!(names(&found), vec!["Alpha"]);
    }

    #[test]
    fn exclude_contested_drops_an_entry_that_has_no_source_yet() {
        // Gamma carries neither value. Testing for "not contested" would let
        // it through, which is the wrong way for a filter whose job is to hold
        // weakly backed entries back.
        let conn = fixture();
        let found = search(
            &conn,
            Some(PatternType::Language),
            &SearchFilters {
                exclude_contested: true,
                ..SearchFilters::default()
            },
        )
        .unwrap();
        assert!(
            !names(&found).contains(&"Gamma"),
            "an entry without a source must not pass the filter"
        );
    }

    #[test]
    fn an_unset_exclude_contested_returns_every_entry() {
        let conn = fixture();
        let found = search(&conn, Some(PatternType::Language), &filters()).unwrap();
        assert_eq!(names(&found), vec!["Alpha", "Beta", "Gamma"]);
    }

    #[test]
    fn free_text_reaches_the_source_so_one_work_can_be_traced() {
        // What cites Oyelaran is a query, not a grep over the catalogue.
        let conn = fixture();
        let found = search(
            &conn,
            None,
            &SearchFilters {
                text: Some("Oyelaran".to_owned()),
                ..SearchFilters::default()
            },
        )
        .unwrap();
        assert_eq!(names(&found), vec!["Beta"]);
    }

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
    fn attachment_filter_matches_exactly() {
        let conn = fixture();
        let found = search(
            &conn,
            None,
            &SearchFilters {
                attachment: Some("subject".to_owned()),
                ..filters()
            },
        )
        .unwrap();
        assert_eq!(names(&found), vec!["Alpha", "Gamma"]);

        let partial = search(
            &conn,
            None,
            &SearchFilters {
                attachment: Some("subj".to_owned()),
                ..filters()
            },
        )
        .unwrap();
        assert!(partial.is_empty(), "attachment is exact, not a substring");
    }

    #[test]
    fn forced_choice_filter_matches_a_substring() {
        let conn = fixture();
        let found = search(
            &conn,
            None,
            &SearchFilters {
                forced_choice: Some("act was willed".to_owned()),
                ..filters()
            },
        )
        .unwrap();
        assert_eq!(names(&found), vec!["Alpha", "Gamma"]);
    }

    #[test]
    fn a_forced_choice_is_shared_by_the_patterns_that_force_it() {
        // The point of the column: it is a key, not a description. Two
        // patterns forcing the same choice at the same attachment carry the
        // same string, so the collision is found by comparing, not by reading.
        let conn = fixture();
        let all = search(&conn, None, &filters()).unwrap();
        let colliding: Vec<&str> = all
            .iter()
            .filter(|p| {
                p.attachment == "subject" && p.forced_choice == "whether the act was willed"
            })
            .map(|p| p.name.as_str())
            .collect();
        assert_eq!(colliding, vec!["Alpha", "Gamma"]);
    }

    #[test]
    fn every_seeded_row_names_a_forced_choice_and_an_attachment() {
        // Both columns are the basis for deciding whether two patterns may be
        // combined, so an empty cell would silently make a pattern collide
        // with every other empty one.
        const ATTACHMENTS: [&str; 11] = [
            "verb",
            "subject",
            "object",
            "noun",
            "possessive",
            "person",
            "spatial frame",
            "connective",
            "word order",
            "whole passage",
            "surface",
        ];
        let conn = seeded();
        for pattern in search(&conn, None, &filters()).unwrap() {
            assert!(
                !pattern.forced_choice.is_empty(),
                "{}: no forced_choice",
                pattern.name
            );
            assert!(
                ATTACHMENTS.contains(&pattern.attachment.as_str()),
                "{}: attachment '{}' is outside the closed vocabulary",
                pattern.name,
                pattern.attachment
            );
        }
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
            for attachment in &facet.attachments {
                expect_hit(
                    SearchFilters {
                        attachment: Some(attachment.clone()),
                        ..filters()
                    },
                    &format!("attachment '{attachment}'"),
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

    // ---- routing: substitution logic --------------------------------------

    fn lens(position: u32, name: &str) -> RosterLens {
        RosterLens {
            position,
            name: name.to_owned(),
            kind: PatternType::Language,
            role: "ablation".to_owned(),
            polarity: "destructive".to_owned(),
            replaces: None,
            reason: None,
        }
    }

    fn substitution(removed: &str, replacement: &str) -> Substitution {
        Substitution {
            removed: removed.to_owned(),
            replacement: replacement.to_owned(),
            replacement_kind: "language".to_owned(),
            reason: "test reason".to_owned(),
        }
    }

    #[test]
    fn apply_substitutions_without_rows_returns_the_roster_unchanged() {
        let roster = vec![lens(1, "Alpha"), lens(2, "Beta")];

        let (lenses, dropped) = apply_substitutions(roster, &[]).unwrap();

        assert_eq!(
            lenses.iter().map(|l| l.name.as_str()).collect::<Vec<_>>(),
            ["Alpha", "Beta"]
        );
        assert!(lenses[0].replaces.is_none());
        assert!(dropped.is_empty());
    }

    #[test]
    fn apply_substitutions_puts_the_replacement_into_the_removed_slot() {
        let roster = vec![lens(1, "Alpha"), lens(2, "Beta")];

        let form = Substitution {
            replacement_kind: "form".to_owned(),
            ..substitution("Alpha", "Gamma")
        };

        let (lenses, _) = apply_substitutions(roster, &[form]).unwrap();

        assert_eq!(lenses[0].name, "Gamma");
        assert_eq!(lenses[0].position, 1);
        assert_eq!(lenses[0].role, "ablation");
        assert_eq!(lenses[0].polarity, "destructive");
        assert_eq!(lenses[0].kind, PatternType::Form);
        assert_eq!(lenses[0].replaces.as_deref(), Some("Alpha"));
        assert_eq!(lenses[0].reason.as_deref(), Some("test reason"));
    }

    #[test]
    fn apply_substitutions_leaves_the_untouched_slots_alone() {
        let roster = vec![lens(1, "Alpha"), lens(2, "Beta")];

        let (lenses, _) = apply_substitutions(roster, &[substitution("Alpha", "Gamma")]).unwrap();

        assert_eq!(lenses[1].name, "Beta");
        assert!(lenses[1].replaces.is_none());
    }

    #[test]
    fn apply_substitutions_with_an_empty_replacement_drops_the_slot_with_its_reason() {
        let roster = vec![lens(1, "Alpha"), lens(2, "Beta")];

        let (lenses, dropped) = apply_substitutions(roster, &[substitution("Beta", "")]).unwrap();

        assert_eq!(
            lenses.iter().map(|l| l.name.as_str()).collect::<Vec<_>>(),
            ["Alpha"]
        );
        assert_eq!(dropped.len(), 1);
        assert_eq!(dropped[0].name, "Beta");
        assert_eq!(dropped[0].reason, "test reason");
    }

    #[test]
    fn apply_substitutions_for_a_lens_not_in_the_roster_changes_nothing() {
        let roster = vec![lens(1, "Alpha")];

        let (lenses, dropped) =
            apply_substitutions(roster, &[substitution("Omega", "Gamma")]).unwrap();

        assert_eq!(lenses[0].name, "Alpha");
        assert!(dropped.is_empty());
    }

    #[test]
    fn apply_substitutions_rejects_a_replacement_of_unknown_kind() {
        let roster = vec![lens(1, "Alpha")];
        let broken = Substitution {
            replacement_kind: String::new(),
            ..substitution("Alpha", "Gamma")
        };

        let message = apply_substitutions(roster, &[broken])
            .unwrap_err()
            .to_string();

        assert!(message.contains("unknown pattern kind"), "got: {message}");
    }

    #[test]
    fn parse_kind_reads_both_kinds() {
        assert_eq!(parse_kind("form").unwrap(), PatternType::Form);
        assert_eq!(parse_kind("language").unwrap(), PatternType::Language);
    }

    #[test]
    fn parse_kind_rejects_an_unknown_or_differently_cased_kind() {
        assert!(parse_kind("Language").is_err());
        assert!(parse_kind("").is_err());
        assert!(parse_kind("dialect").is_err());
    }

    // ---- routing: fixture edge cases --------------------------------------

    /// One combination with two lenses and one profile. `Elsewhere` has no
    /// routing row, `Broken` substitutes a lens that is not in the catalogue.
    const ROUTING_FIXTURE_SQL: &str = "
INSERT INTO combinations (id, name, axis, trigger_text, provenance)
VALUES ('K1', 'base', 'Causality', 'any sentence', 'test');
INSERT INTO combination_lenses (combination, position, name, kind, role, polarity)
VALUES ('K1', 1, 'Alpha', 'language', 'ablation', 'destructive'),
       ('K1', 2, 'Haiku', 'form', 'reconstruction', 'constructive');
INSERT INTO routing_profiles (source_language, own_entries, note)
VALUES ('Fixture', '[\"Alpha\"]', 'a note'),
       ('Elsewhere', '[]', ''),
       ('Broken', 'not json', '');
INSERT INTO routing (source_language, combination, verdict, marker, note, citation, provenance)
VALUES ('Fixture', 'K1', 'fires', 'the marker', '', 'A Grammar', 'derived'),
       ('Broken', 'K1', 'fires', 'm', '', 'c', 'derived');
INSERT INTO substitutions
    (source_language, combination, removed, replacement, replacement_kind, reason)
VALUES ('Fixture', 'K1', 'Alpha', 'Beta', 'language', 'own entry'),
       ('Broken', 'K1', 'Alpha', 'Nowhere', 'form', 'test');
";

    fn routing_fixture() -> Connection {
        let conn = fixture();
        conn.execute_batch(ROUTING_SCHEMA_SQL)
            .expect("routing schema failed");
        conn.execute_batch(ROUTING_FIXTURE_SQL)
            .expect("routing fixture failed");
        conn
    }

    #[test]
    fn routing_takes_the_replacement_kind_from_the_substitution_row() {
        let conn = routing_fixture();

        let profile = routing(&conn, "Fixture")
            .unwrap()
            .expect("profile expected");

        let roster = &profile.combinations[0].roster;
        assert_eq!(roster[0].name, "Beta");
        assert_eq!(roster[0].kind, PatternType::Language);
        assert_eq!(roster[1].kind, PatternType::Form);
    }

    #[test]
    fn routing_matches_the_profile_without_regard_to_case_or_padding() {
        let conn = routing_fixture();

        let profile = routing(&conn, "  fIXTURE ")
            .unwrap()
            .expect("profile expected");

        assert_eq!(profile.source_language, "Fixture");
        assert_eq!(profile.own_entries, ["Alpha"]);
        assert_eq!(profile.note, "a note");
    }

    #[test]
    fn routing_for_an_unknown_language_is_none() {
        let conn = routing_fixture();

        assert!(routing(&conn, "Klingon").unwrap().is_none());
        assert!(routing(&conn, "").unwrap().is_none());
    }

    #[test]
    fn routing_reports_a_missing_row_as_unrouted_instead_of_skipping_it() {
        let conn = routing_fixture();

        let profile = routing(&conn, "Elsewhere")
            .unwrap()
            .expect("profile expected");

        assert_eq!(profile.combinations.len(), 1);
        assert_eq!(profile.combinations[0].verdict, "unrouted");
        assert_eq!(profile.combinations[0].roster[0].name, "Alpha");
    }

    #[test]
    fn routing_still_routes_a_replacement_missing_from_the_catalogue() {
        let conn = routing_fixture();

        let profile = routing(&conn, "Broken").unwrap().expect("profile expected");

        let roster = &profile.combinations[0].roster;
        assert_eq!(roster[0].name, "Nowhere");
        assert_eq!(roster[0].kind, PatternType::Form);
    }

    #[test]
    fn routing_names_the_lenses_this_database_lacks() {
        let conn = routing_fixture();

        let profile = routing(&conn, "Broken").unwrap().expect("profile expected");

        assert_eq!(profile.missing_from_catalogue, ["Nowhere"]);
    }

    #[test]
    fn routing_reports_no_missing_lens_for_a_profile_whose_lenses_all_exist() {
        let conn = routing_fixture();

        let profile = routing(&conn, "Fixture")
            .unwrap()
            .expect("profile expected");

        assert!(profile.missing_from_catalogue.is_empty());
    }

    #[test]
    fn missing_lenses_lists_every_routed_name_the_pattern_tables_lack() {
        let conn = routing_fixture();

        assert_eq!(missing_lenses(&conn).unwrap(), ["Nowhere"]);
    }

    #[test]
    fn routing_treats_a_broken_own_entries_cell_as_empty() {
        let conn = routing_fixture();
        conn.execute_batch("DELETE FROM substitutions WHERE source_language = 'Broken';")
            .unwrap();

        let profile = routing(&conn, "Broken").unwrap().expect("profile expected");

        assert!(profile.own_entries.is_empty());
    }

    #[test]
    fn combinations_fail_on_an_unknown_lens_kind() {
        let conn = routing_fixture();
        conn.execute_batch("UPDATE combination_lenses SET kind = 'dialect' WHERE position = 1;")
            .unwrap();

        let message = combinations(&conn).unwrap_err().to_string();

        assert!(message.contains("dialect"), "got: {message}");
    }

    #[test]
    fn routing_overview_carries_the_message_profiles_and_neutral_rosters() {
        let conn = routing_fixture();

        let overview = routing_overview(&conn, Some("hint".to_owned())).unwrap();

        assert_eq!(overview.message.as_deref(), Some("hint"));
        assert_eq!(overview.profiles, ["Fixture", "Elsewhere", "Broken"]);
        assert_eq!(overview.combinations[0].roster[0].name, "Alpha");
    }

    // ---- routing: the real seed --------------------------------------------

    #[test]
    fn prepare_adds_the_routing_to_a_database_that_predates_it() {
        let mut conn = Connection::open_in_memory().unwrap();
        conn.execute_batch(SCHEMA_SQL).unwrap();
        conn.execute_batch(SEED_SQL).unwrap();
        let before = search(&conn, None, &filters()).unwrap().len();

        let seeded = prepare(&mut conn).unwrap();

        assert_eq!(
            seeded,
            Seeded {
                patterns: false,
                routing: true
            }
        );
        assert_eq!(search(&conn, None, &filters()).unwrap().len(), before);
        assert_eq!(combinations(&conn).unwrap().len(), 8);
    }

    #[test]
    fn a_database_seeded_before_a_replacement_entered_the_catalogue_still_routes() {
        // A database seeded by an older build can lack a lens the routing
        // substitutes in; 0.6.2 lacked the one the Japanese K4 slot first got.
        // That used to fail the whole profile. Simulated here by removing the
        // current replacement.
        let mut conn = Connection::open_in_memory().unwrap();
        conn.execute_batch(SCHEMA_SQL).unwrap();
        conn.execute_batch(SEED_SQL).unwrap();
        conn.execute_batch("DELETE FROM languages WHERE name = 'Hausa (pluractional verbs)';")
            .unwrap();
        prepare(&mut conn).unwrap();

        let profile = routing(&conn, "Japanese")
            .unwrap()
            .expect("profile expected");

        assert_eq!(
            profile.combinations[3].roster[0].name,
            "Hausa (pluractional verbs)"
        );
        assert_eq!(
            profile.missing_from_catalogue,
            ["Hausa (pluractional verbs)"]
        );
    }

    #[test]
    fn the_seed_names_no_lens_its_own_catalogue_lacks() {
        let conn = seeded();

        assert!(missing_lenses(&conn).unwrap().is_empty());
    }

    #[test]
    fn every_seeded_replacement_kind_matches_the_table_it_lives_in() {
        let conn = seeded();

        let mismatched: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM substitutions s WHERE s.replacement <> '' AND NOT ( \
                 (s.replacement_kind = 'language' AND EXISTS \
                  (SELECT 1 FROM languages l WHERE l.name = s.replacement)) OR \
                 (s.replacement_kind = 'form' AND EXISTS \
                  (SELECT 1 FROM forms f WHERE f.name = s.replacement)))",
                [],
                |row| row.get(0),
            )
            .unwrap();

        assert_eq!(mismatched, 0);
    }

    #[test]
    fn the_russian_profile_keeps_the_polish_lenses_russian_grammar_leaves_open() {
        // Regression: Russian agreement after five is free and its predicate
        // case does not split nouns from adjectives, so neither Polish lens
        // is redundant for a Russian text.
        let conn = seeded();

        let profile = routing(&conn, "Russian")
            .unwrap()
            .expect("profile expected");

        assert_eq!(
            profile.combinations[3].roster[1].name,
            "Polish (numeral threshold)"
        );
        assert_eq!(
            profile.combinations[7].roster[2].name,
            "Polish (instrumental predication)"
        );
    }

    #[test]
    fn the_seed_holds_seven_profiles_in_order() {
        let conn = seeded();

        assert_eq!(
            routing_profiles(&conn).unwrap(),
            [
                "German", "English", "French", "Spanish", "Russian", "Polish", "Japanese"
            ]
        );
    }

    #[test]
    fn every_seeded_roster_lens_exists_in_its_table_and_is_sourced() {
        let conn = seeded();

        let orphans: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM combination_lenses c WHERE NOT EXISTS ( \
                 SELECT 1 FROM languages l WHERE l.name = c.name AND c.kind = 'language' \
                 AND l.status = 'sourced' UNION ALL \
                 SELECT 1 FROM forms f WHERE f.name = c.name AND c.kind = 'form' \
                 AND f.status = 'sourced')",
                [],
                |row| row.get(0),
            )
            .unwrap();

        assert_eq!(orphans, 0);
    }

    #[test]
    fn every_seeded_replacement_exists_and_is_sourced() {
        let conn = seeded();

        let orphans: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM substitutions s WHERE s.replacement <> '' AND NOT EXISTS ( \
                 SELECT 1 FROM languages l WHERE l.name = s.replacement AND l.status = 'sourced' \
                 UNION ALL \
                 SELECT 1 FROM forms f WHERE f.name = s.replacement AND f.status = 'sourced')",
                [],
                |row| row.get(0),
            )
            .unwrap();

        assert_eq!(orphans, 0);
    }

    #[test]
    fn every_seeded_profile_routes_every_combination() {
        let conn = seeded();

        let gaps: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM routing_profiles p, combinations c WHERE NOT EXISTS ( \
                 SELECT 1 FROM routing r WHERE r.source_language = p.source_language \
                 AND r.combination = c.id)",
                [],
                |row| row.get(0),
            )
            .unwrap();

        assert_eq!(gaps, 0);
    }

    #[test]
    fn the_german_profile_substitutes_only_ancient_greek_in_k8() {
        // The combinations were derived on German; the one change the check
        // found is a K8 lens whose device German shares.
        let conn = seeded();

        let substituted: Vec<(String, String)> = conn
            .prepare(
                "SELECT combination, removed FROM substitutions \
                 WHERE source_language = 'German'",
            )
            .unwrap()
            .query_map([], |row| Ok((row.get(0)?, row.get(1)?)))
            .unwrap()
            .collect::<std::result::Result<_, _>>()
            .unwrap();

        assert_eq!(
            substituted,
            [(
                "K8".to_string(),
                "Ancient Greek (article & substantivization)".to_string()
            )]
        );
    }

    #[test]
    fn the_russian_profile_inverts_k5_and_replaces_its_own_entry() {
        let conn = seeded();

        let profile = routing(&conn, "Russian")
            .unwrap()
            .expect("profile expected");
        let k5 = &profile.combinations[4];

        assert_eq!(k5.id, "K5");
        assert_eq!(k5.verdict, "inverted");
        assert_eq!(k5.roster[0].name, "Hindi-Urdu (vector verbs)");
        assert_eq!(k5.roster[0].replaces.as_deref(), Some("Russian"));
        assert_eq!(k5.roster[0].role, "ablation");
    }

    #[test]
    fn the_polish_profile_drops_mongolian_from_k6() {
        let conn = seeded();

        let profile = routing(&conn, "Polish").unwrap().expect("profile expected");
        let k6 = &profile.combinations[5];

        assert_eq!(
            k6.roster
                .iter()
                .map(|l| l.name.as_str())
                .collect::<Vec<_>>(),
            [
                "Navajo (inalienable possession)",
                "Hawaiian (a/o possession)",
                "Pohnpeian (possessive classifiers)"
            ]
        );
        assert_eq!(k6.dropped[0].name, "Mongolian (reflexive possession)");
    }

    #[test]
    fn the_polish_profile_excludes_all_three_polish_entries() {
        let conn = seeded();

        let profile = routing(&conn, "Polish").unwrap().expect("profile expected");

        assert_eq!(
            profile.own_entries,
            [
                "Polish (instrumental predication)",
                "Polish (numeral threshold)",
                "Polish (masculine-personal plural)"
            ]
        );
    }

    #[test]
    fn the_japanese_profile_replaces_yucatec_with_hausa_and_keeps_hungarian() {
        // The 100-sentence check: Yucatec forced no new choice on Japanese
        // (0/10), Hausa did (8/10); Hungarian still did too (8/10), so the
        // earlier `-tachi` substitution was withdrawn.
        let conn = seeded();

        let profile = routing(&conn, "Japanese")
            .unwrap()
            .expect("profile expected");
        let k4 = &profile.combinations[3];

        assert_eq!(
            k4.roster
                .iter()
                .map(|l| l.name.as_str())
                .collect::<Vec<_>>(),
            [
                "Hausa (pluractional verbs)",
                "Polish (numeral threshold)",
                "Hungarian (associative plural)",
                "Mojeño Trinitario (possessive classes)",
                "Toki Pona",
                "Swahili (noun classes)"
            ]
        );
    }

    #[test]
    fn the_neutral_k4_roster_closes_on_a_reconstruction_after_its_reducer() {
        // K1 runs on every sentence, so its reducer always displaces Toki
        // Pona; without a closer of its own K4 would end purely destructive.
        let conn = seeded();

        let overview = routing_overview(&conn, None).unwrap();
        let k4 = &overview.combinations[3];
        let last = k4.roster.last().expect("roster expected");

        assert_eq!(
            (
                last.name.as_str(),
                last.role.as_str(),
                last.polarity.as_str()
            ),
            ("Swahili (noun classes)", "reconstruction", "constructive")
        );
    }

    #[test]
    fn the_french_profile_replaces_russian_in_k5_with_hindi_urdu() {
        // The 100-sentence check: the passé composé against the imparfait
        // already forced the choice in 9 of 10, so Russian added nothing.
        let conn = seeded();

        let profile = routing(&conn, "French").unwrap().expect("profile expected");
        let k5 = &profile.combinations[4];

        assert_eq!(k5.verdict, "fires");
        assert_eq!(k5.roster[0].name, "Hindi-Urdu (vector verbs)");
        assert_eq!(k5.roster[0].replaces.as_deref(), Some("Russian"));
        assert!(k5.dropped.is_empty());
    }

    #[test]
    fn the_german_and_spanish_profiles_drop_ancient_greek_from_k8() {
        let conn = seeded();

        let german = routing(&conn, "German").unwrap().expect("profile expected");
        let spanish = routing(&conn, "Spanish")
            .unwrap()
            .expect("profile expected");

        assert_eq!(
            german.combinations[7].dropped[0].name,
            "Ancient Greek (article & substantivization)"
        );
        assert_eq!(
            spanish.combinations[7].dropped[0].name,
            "Ancient Greek (article & substantivization)"
        );
    }

    #[test]
    fn the_english_profile_keeps_ancient_greek_in_k8() {
        // Only the profiles whose check found the device shared drop it; the
        // substitution must not leak into a profile that has no row for it.
        let conn = seeded();

        let profile = routing(&conn, "English")
            .unwrap()
            .expect("profile expected");
        let k8 = &profile.combinations[7];

        assert_eq!(
            k8.roster[1].name,
            "Ancient Greek (article & substantivization)"
        );
        assert!(k8.dropped.is_empty());
    }

    #[test]
    fn the_slavic_profiles_drop_finnish_from_k5() {
        let conn = seeded();

        let polish = routing(&conn, "Polish").unwrap().expect("profile expected");
        let russian = routing(&conn, "Russian")
            .unwrap()
            .expect("profile expected");

        assert_eq!(
            polish.combinations[4].dropped[0].name,
            "Finnish (partitive object)"
        );
        assert_eq!(
            russian.combinations[4].dropped[0].name,
            "Finnish (partitive object)"
        );
    }

    #[test]
    fn every_seeded_row_but_k1_carries_the_checked_provenance() {
        let conn = seeded();

        let unchecked: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM routing WHERE combination <> 'K1' \
                 AND provenance <> 'checked'",
                [],
                |row| row.get(0),
            )
            .unwrap();

        assert_eq!(unchecked, 0);
    }

    #[test]
    fn the_french_profile_has_no_own_entries_and_leaves_k3_untouched() {
        let conn = seeded();

        let profile = routing(&conn, "French").unwrap().expect("profile expected");

        assert!(profile.own_entries.is_empty());
        assert_eq!(profile.combinations[2].roster[0].name, "Tuyuca");
        assert!(profile.combinations[2].dropped.is_empty());
    }

    // ---- routing: fuzzing ---------------------------------------------------

    /// xorshift64*: enough to spread inputs, no dependency needed.
    struct Rng(u64);

    impl Rng {
        fn next(&mut self) -> u64 {
            self.0 ^= self.0 >> 12;
            self.0 ^= self.0 << 25;
            self.0 ^= self.0 >> 27;
            self.0.wrapping_mul(0x2545_F491_4F6C_DD1D)
        }
    }

    /// A fresh seed per run unless `LAT_FUZZ_SEED` pins one to reproduce a
    /// finding.
    fn fuzz_seed() -> u64 {
        use std::hash::{BuildHasher, RandomState};
        std::env::var("LAT_FUZZ_SEED")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or_else(|| RandomState::new().hash_one(std::process::id()) | 1)
    }

    /// Run count from `LAT_FUZZ_RUNS`; small by default so the suite stays
    /// fast. Acceptance runs set it to 20000 or more.
    fn fuzz_runs() -> u64 {
        std::env::var("LAT_FUZZ_RUNS")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(300)
    }

    /// Quotes, wildcards, comment and statement syntax, case variants of real
    /// profiles, whitespace, control characters and non-ASCII text.
    const FUZZ_PIECES: [&str; 22] = [
        "'",
        "\"",
        "%",
        "_",
        "--",
        ";",
        "/*",
        "*/",
        " OR ",
        "1=1",
        "\0",
        "\n",
        "\t",
        " ",
        "German",
        "rUSSIAN",
        "japanese",
        "ß",
        "日本語",
        "😀",
        "\u{202e}",
        "Polish (numeral threshold)",
    ];

    #[test]
    fn fuzz_routing_never_errors_and_only_returns_known_profiles() {
        let conn = seeded();
        let profiles = routing_profiles(&conn).unwrap();
        let seed = fuzz_seed();
        let mut rng = Rng(seed);

        for run in 0..fuzz_runs() {
            let len = (rng.next() % 12) as usize;
            let input: String = (0..len)
                .map(|_| FUZZ_PIECES[(rng.next() % FUZZ_PIECES.len() as u64) as usize])
                .collect();

            let result = routing(&conn, &input);

            let found = result.unwrap_or_else(|e| {
                panic!("run {run}, LAT_FUZZ_SEED={seed}: {input:?} raised {e}")
            });
            if let Some(profile) = found {
                assert!(
                    profiles.contains(&profile.source_language),
                    "run {run}, LAT_FUZZ_SEED={seed}: {input:?} returned {:?}",
                    profile.source_language
                );
                assert_eq!(
                    profile.source_language.to_lowercase(),
                    input.trim().to_lowercase(),
                    "run {run}, LAT_FUZZ_SEED={seed}: {input:?} matched a different profile"
                );
            }
        }
    }
}
