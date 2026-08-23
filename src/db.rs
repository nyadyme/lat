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

/// Creates the schema and seeds the database on first start.
/// Returns the resolved path.
pub fn init() -> Result<PathBuf> {
    let path = db_path()?;
    let mut conn = open(&path)?;
    conn.execute_batch(SCHEMA_SQL)
        .context("could not create schema")?;

    let count: i64 = conn.query_row(
        "SELECT (SELECT COUNT(*) FROM forms) + (SELECT COUNT(*) FROM languages)",
        [],
        |row| row.get(0),
    )?;
    if count == 0 {
        // Seed atomically: if seeding aborts midway, everything is rolled back
        // so no half-filled database is left behind that would wrongly count as
        // "already seeded" (count != 0) on the next start.
        let tx = conn.transaction()?;
        tx.execute_batch(SEED_SQL)
            .context("could not insert example data")?;
        tx.commit()?;
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
