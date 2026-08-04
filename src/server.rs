// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 Dana Schlifka

//! MCP server with four read-only tools over the pattern database.

use std::path::PathBuf;
use std::sync::Arc;

use rmcp::{
    ErrorData as McpError, ServerHandler,
    handler::server::{router::tool::ToolRouter, wrapper::Parameters},
    model::*,
    schemars, tool, tool_handler, tool_router,
};
use serde::Deserialize;

use crate::db;
use crate::models::{PatternType, SearchFilters};

const INSTRUCTIONS: &str = "\
A toolbox of thinking patterns: natural languages (grammatical reflexes) and \
bound poetic forms or writing techniques. Each pattern forces a different \
structure of thought and thereby makes hidden aspects of a subject visible. \
Use 'search_patterns' to find patterns that fit a problem theme (e.g. \
causality, coexistence, perspective), 'list_facets' to see valid filter \
values, 'list_patterns' for an overview, and 'get_pattern' for the full \
details of a pattern that is then to be applied.";

/// Parameters for the search. All fields optional, combined with AND.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct SearchParams {
    /// Pattern kind: 'form', 'language', or omit for both.
    pub kind: Option<PatternType>,
    /// Exact tag (an element of the tags array).
    pub tag: Option<String>,
    /// Exact theme, e.g. 'Causality' or 'Coexistence'.
    pub theme: Option<String>,
    /// Exact category, e.g. 'Poetic form', 'Register' or 'Technique'.
    pub category: Option<String>,
    /// Exact classification.
    pub classification: Option<String>,
    /// Substring that should appear in the focus field.
    pub focus: Option<String>,
    /// Free text across name, description, feature and tags.
    pub text: Option<String>,
    /// Names to exclude, e.g. the user's own language, so that contrasting
    /// lenses surface rather than the structure they already think in.
    pub exclude_names: Option<Vec<String>>,
}

impl SearchParams {
    fn split(self) -> (Option<PatternType>, SearchFilters) {
        (
            self.kind,
            SearchFilters {
                tag: self.tag,
                theme: self.theme,
                category: self.category,
                classification: self.classification,
                focus: self.focus,
                text: self.text,
                exclude_names: self.exclude_names.unwrap_or_default(),
            },
        )
    }
}

/// Parameters for fetching a single pattern.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct GetParams {
    /// Pattern kind: 'form' or 'language'.
    pub kind: PatternType,
    /// Exact name of the pattern.
    pub name: String,
}

/// Parameters for listing / facets, optionally restricted to one kind.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct ListParams {
    /// Optional pattern kind ('form' or 'language') to restrict to.
    pub kind: Option<PatternType>,
}

/// The MCP server. Holds only the database path; connections are opened per
/// call.
#[derive(Clone)]
pub struct LatServer {
    db_path: Arc<PathBuf>,
    // Used by the #[tool_router]/#[tool_handler] macros via generated code.
    #[allow(dead_code)]
    tool_router: ToolRouter<LatServer>,
}

impl LatServer {
    /// Runs a blocking database operation on a worker thread.
    async fn run_db<T, F>(&self, f: F) -> Result<T, McpError>
    where
        F: FnOnce(&rusqlite::Connection) -> anyhow::Result<T> + Send + 'static,
        T: Send + 'static,
    {
        let path = self.db_path.clone();
        tokio::task::spawn_blocking(move || {
            let conn = db::open(&path)?;
            f(&conn)
        })
        .await
        .map_err(|e| McpError::internal_error(format!("task error: {e}"), None))?
        .map_err(|e| McpError::internal_error(e.to_string(), None))
    }
}

#[tool_router]
impl LatServer {
    /// Creates the server for the given database file.
    pub fn new(db_path: PathBuf) -> Self {
        Self {
            db_path: Arc::new(db_path),
            tool_router: Self::tool_router(),
        }
    }

    #[tool(
        description = "Searches thinking patterns (languages and/or forms) by filters. All \
                       filters are optional and combined with AND. Ideal for finding patterns \
                       that fit a problem theme. Use 'exclude_names' to drop the user's own \
                       source language so contrasting lenses surface instead of the structure \
                       they already think in."
    )]
    async fn search_patterns(
        &self,
        Parameters(params): Parameters<SearchParams>,
    ) -> Result<CallToolResult, McpError> {
        let (kind, filters) = params.split();
        let results = self
            .run_db(move |conn| db::search(conn, kind, &filters))
            .await?;
        json_result(&results)
    }

    #[tool(description = "Returns the full details of a pattern by kind and exact name.")]
    async fn get_pattern(
        &self,
        Parameters(params): Parameters<GetParams>,
    ) -> Result<CallToolResult, McpError> {
        let GetParams { kind, name } = params;
        let lookup = name.clone();
        let found = self.run_db(move |conn| db::get(conn, kind, &lookup)).await?;
        match found {
            Some(pattern) => json_result(&pattern),
            None => Ok(CallToolResult::success(vec![ContentBlock::text(format!(
                "No pattern '{name}' of kind {kind:?} found."
            ))])),
        }
    }

    #[tool(
        description = "Lists all patterns, optionally restricted to one kind ('form' or \
                       'language')."
    )]
    async fn list_patterns(
        &self,
        Parameters(params): Parameters<ListParams>,
    ) -> Result<CallToolResult, McpError> {
        let kind = params.kind;
        let results = self
            .run_db(move |conn| db::search(conn, kind, &SearchFilters::default()))
            .await?;
        json_result(&results)
    }

    #[tool(
        description = "Shows the available filter values (categories, classifications, tags, \
                       themes) per table, so valid filters for 'search_patterns' are known."
    )]
    async fn list_facets(
        &self,
        Parameters(params): Parameters<ListParams>,
    ) -> Result<CallToolResult, McpError> {
        let kind = params.kind;
        let facets = self.run_db(move |conn| db::facets(conn, kind)).await?;
        json_result(&facets)
    }
}

#[tool_handler]
impl ServerHandler for LatServer {
    fn get_info(&self) -> ServerInfo {
        let mut implementation = Implementation::from_build_env();
        implementation.name = env!("CARGO_PKG_NAME").to_string();
        implementation.version = env!("CARGO_PKG_VERSION").to_string();

        ServerInfo::new(ServerCapabilities::builder().enable_tools().build())
            .with_server_info(implementation)
            .with_protocol_version(ProtocolVersion::V_2024_11_05)
            .with_instructions(INSTRUCTIONS)
    }
}

/// Serializes a value as pretty JSON into a tool result.
fn json_result<T: serde::Serialize>(value: &T) -> Result<CallToolResult, McpError> {
    let text = serde_json::to_string_pretty(value)
        .map_err(|e| McpError::internal_error(e.to_string(), None))?;
    Ok(CallToolResult::success(vec![ContentBlock::text(text)]))
}
