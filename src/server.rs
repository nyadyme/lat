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
details of a pattern that is then to be applied. When combining several \
patterns, read 'forced_choice' and 'attachment': two patterns that force the \
same choice at the same attachment point yield correlated findings, so take \
at most one of them.";

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
    /// Substring in the forced_choice field: the choice a pattern makes
    /// obligatory, e.g. 'source of the information'.
    pub forced_choice: Option<String>,
    /// Exact attachment: the constituent a pattern interrogates, e.g. 'verb',
    /// 'subject', 'noun', 'possessive', 'whole passage'. See list_facets.
    pub attachment: Option<String>,
    /// Free text across name, description, feature, tags and classification.
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
                forced_choice: self.forced_choice,
                attachment: self.attachment,
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
        let found = self
            .run_db(move |conn| db::get(conn, kind, &lookup))
            .await?;
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
        description = "Shows the available filter values (categories, classifications, tags \
                       and themes) per table, so valid filters for \
                       'search_patterns' are known."
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

#[cfg(test)]
mod tests {
    use serde_json::{Value, json};

    use super::*;
    use crate::db::testing::TempDb;

    /// A server over a freshly seeded database file.
    fn server() -> (TempDb, LatServer) {
        let db = TempDb::new();
        let server = LatServer::new(db.path().to_path_buf());
        (db, server)
    }

    /// Concatenates the text blocks of a tool result.
    fn text_of(result: &CallToolResult) -> String {
        result
            .content
            .iter()
            .filter_map(|block| block.as_text())
            .map(|text| text.text.as_str())
            .collect::<Vec<_>>()
            .join("")
    }

    /// Parses the text blocks of a tool result as JSON.
    fn json_of(result: &CallToolResult) -> Value {
        serde_json::from_str(&text_of(result)).expect("the tool result should be JSON")
    }

    fn names_of(result: &CallToolResult) -> Vec<String> {
        json_of(result)
            .as_array()
            .expect("expected a JSON array")
            .iter()
            .map(|item| item["name"].as_str().expect("missing name").to_owned())
            .collect()
    }

    fn search_params(value: Value) -> SearchParams {
        serde_json::from_value(value).expect("the search parameters should deserialize")
    }

    // ---- parameter parsing -----------------------------------------------

    #[test]
    fn search_parameters_are_all_optional() {
        let params = search_params(json!({}));
        let (kind, filters) = params.split();

        assert!(kind.is_none());
        assert!(filters.tag.is_none());
        assert!(filters.theme.is_none());
        assert!(filters.category.is_none());
        assert!(filters.classification.is_none());
        assert!(filters.focus.is_none());
        assert!(filters.text.is_none());
        assert!(
            filters.exclude_names.is_empty(),
            "a missing exclude_names must not exclude anything"
        );
    }

    #[test]
    fn search_parameters_are_split_into_kind_and_filters() {
        let params = search_params(json!({
            "kind": "language",
            "tag": "ergative",
            "theme": "Causality",
            "category": "Language",
            "classification": "isolate",
            "focus": "chain",
            "text": "aspect",
            "exclude_names": ["German", "English"],
        }));
        let (kind, filters) = params.split();

        assert_eq!(kind, Some(PatternType::Language));
        assert_eq!(filters.tag.as_deref(), Some("ergative"));
        assert_eq!(filters.theme.as_deref(), Some("Causality"));
        assert_eq!(filters.category.as_deref(), Some("Language"));
        assert_eq!(filters.classification.as_deref(), Some("isolate"));
        assert_eq!(filters.focus.as_deref(), Some("chain"));
        assert_eq!(filters.text.as_deref(), Some("aspect"));
        assert_eq!(filters.exclude_names, vec!["German", "English"]);
    }

    #[test]
    fn an_unknown_kind_is_rejected_before_the_database_is_touched() {
        assert!(serde_json::from_value::<SearchParams>(json!({"kind": "dialect"})).is_err());
        assert!(
            serde_json::from_value::<GetParams>(json!({"kind": "dialect", "name": "Haiku"}))
                .is_err()
        );
    }

    #[test]
    fn get_parameters_require_a_kind_and_a_name() {
        let params: GetParams =
            serde_json::from_value(json!({"kind": "form", "name": "Haiku"})).unwrap();
        assert_eq!(params.kind, PatternType::Form);
        assert_eq!(params.name, "Haiku");

        assert!(serde_json::from_value::<GetParams>(json!({"kind": "form"})).is_err());
        assert!(serde_json::from_value::<GetParams>(json!({"name": "Haiku"})).is_err());
    }

    #[test]
    fn list_parameters_accept_an_absent_kind() {
        let both: ListParams = serde_json::from_value(json!({})).unwrap();
        assert!(both.kind.is_none());

        let forms: ListParams = serde_json::from_value(json!({"kind": "form"})).unwrap();
        assert_eq!(forms.kind, Some(PatternType::Form));
    }

    // ---- json_result -----------------------------------------------------

    #[test]
    fn json_result_returns_one_pretty_printed_text_block() {
        let result = json_result(&json!({"name": "Haiku"})).unwrap();

        assert_eq!(result.content.len(), 1);
        assert_ne!(result.is_error, Some(true));

        let text = text_of(&result);
        assert!(text.contains('\n'), "expected pretty JSON, got: {text}");
        assert_eq!(json_of(&result), json!({"name": "Haiku"}));
    }

    // ---- get_info --------------------------------------------------------

    #[test]
    fn the_server_announces_its_name_version_tools_and_instructions() {
        let (_db, server) = server();
        let info = server.get_info();

        assert_eq!(info.server_info.name, env!("CARGO_PKG_NAME"));
        assert_eq!(info.server_info.version, env!("CARGO_PKG_VERSION"));
        assert!(info.capabilities.tools.is_some(), "tools must be enabled");
        assert_eq!(info.instructions.as_deref(), Some(INSTRUCTIONS));
    }

    // ---- the tools -------------------------------------------------------

    #[tokio::test]
    async fn list_patterns_returns_both_kinds_by_default() {
        let (_db, server) = server();
        let result = server
            .list_patterns(Parameters(ListParams { kind: None }))
            .await
            .unwrap();

        let patterns = json_of(&result);
        let items = patterns.as_array().unwrap();
        assert!(!items.is_empty());
        assert!(items.iter().any(|item| item["kind"] == "form"));
        assert!(items.iter().any(|item| item["kind"] == "language"));
    }

    #[tokio::test]
    async fn list_patterns_restricted_to_a_kind_returns_only_that_kind() {
        let (_db, server) = server();
        let result = server
            .list_patterns(Parameters(ListParams {
                kind: Some(PatternType::Form),
            }))
            .await
            .unwrap();

        let patterns = json_of(&result);
        let items = patterns.as_array().unwrap();
        assert!(!items.is_empty());
        assert!(items.iter().all(|item| item["kind"] == "form"));
    }

    #[tokio::test]
    async fn list_facets_reports_one_entry_per_kind() {
        let (_db, server) = server();
        let result = server
            .list_facets(Parameters(ListParams { kind: None }))
            .await
            .unwrap();

        let facets = json_of(&result);
        let entries = facets.as_array().unwrap();
        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0]["kind"], "form");
        assert_eq!(entries[1]["kind"], "language");
        assert!(entries[1]["themes"].as_array().unwrap().len() > 1);
    }

    #[tokio::test]
    async fn list_facets_restricted_to_a_kind_reports_that_kind_only() {
        let (_db, server) = server();
        let result = server
            .list_facets(Parameters(ListParams {
                kind: Some(PatternType::Language),
            }))
            .await
            .unwrap();

        let entries = json_of(&result);
        let entries = entries.as_array().unwrap();
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0]["kind"], "language");
    }

    #[tokio::test]
    async fn search_patterns_without_filters_matches_the_full_listing() {
        let (_db, server) = server();

        let searched = server
            .search_patterns(Parameters(search_params(json!({}))))
            .await
            .unwrap();
        let listed = server
            .list_patterns(Parameters(ListParams { kind: None }))
            .await
            .unwrap();

        assert_eq!(names_of(&searched), names_of(&listed));
    }

    #[tokio::test]
    async fn search_patterns_applies_a_theme_filter() {
        let (_db, server) = server();
        let result = server
            .search_patterns(Parameters(search_params(
                json!({"kind": "language", "theme": "Causality"}),
            )))
            .await
            .unwrap();

        let patterns = json_of(&result);
        let items = patterns.as_array().unwrap();
        assert!(!items.is_empty(), "the catalogue should cover Causality");
        for item in items {
            assert_eq!(item["kind"], "language");
            let themes = item["themes"].as_array().unwrap();
            assert!(themes.iter().any(|theme| theme == "Causality"));
        }
    }

    #[tokio::test]
    async fn search_patterns_drops_the_excluded_source_language() {
        let (_db, server) = server();

        let all = server
            .search_patterns(Parameters(search_params(json!({"kind": "language"}))))
            .await
            .unwrap();
        assert!(names_of(&all).contains(&"German".to_owned()));

        let contrasting = server
            .search_patterns(Parameters(search_params(
                json!({"kind": "language", "exclude_names": ["German"]}),
            )))
            .await
            .unwrap();
        let names = names_of(&contrasting);
        assert!(!names.contains(&"German".to_owned()));
        assert_eq!(names.len(), names_of(&all).len() - 1);
    }

    #[tokio::test]
    async fn search_patterns_returns_an_empty_array_when_nothing_matches() {
        let (_db, server) = server();
        let result = server
            .search_patterns(Parameters(search_params(
                json!({"tag": "no-such-tag-exists"}),
            )))
            .await
            .unwrap();

        assert_ne!(result.is_error, Some(true));
        assert_eq!(json_of(&result), json!([]));
    }

    #[tokio::test]
    async fn get_pattern_returns_the_full_record() {
        let (_db, server) = server();
        let result = server
            .get_pattern(Parameters(GetParams {
                kind: PatternType::Language,
                name: "German".to_owned(),
            }))
            .await
            .unwrap();

        let pattern = json_of(&result);
        assert_eq!(pattern["kind"], "language");
        assert_eq!(pattern["name"], "German");
        for field in [
            "description",
            "focus",
            "category",
            "classification",
            "feature",
        ] {
            assert!(pattern[field].is_string(), "{field} should be a string");
        }
        assert!(pattern["tags"].is_array());
        assert!(pattern["themes"].is_array());
    }

    #[tokio::test]
    async fn get_pattern_reports_an_unknown_name_without_failing() {
        let (_db, server) = server();
        let result = server
            .get_pattern(Parameters(GetParams {
                kind: PatternType::Form,
                name: "No Such Form".to_owned(),
            }))
            .await
            .unwrap();

        assert_ne!(
            result.is_error,
            Some(true),
            "a miss is an answer, not a protocol error"
        );
        let text = text_of(&result);
        assert!(text.contains("No Such Form"), "unhelpful message: {text}");
    }

    #[tokio::test]
    async fn get_pattern_does_not_cross_the_two_kinds() {
        let (_db, server) = server();
        let result = server
            .get_pattern(Parameters(GetParams {
                kind: PatternType::Form,
                name: "German".to_owned(),
            }))
            .await
            .unwrap();

        assert!(
            text_of(&result).contains("No pattern"),
            "German is a language, not a form"
        );
    }

    #[tokio::test]
    async fn a_missing_database_file_surfaces_as_a_tool_error() {
        let server = LatServer::new(
            std::env::temp_dir()
                .join("lat-nonexistent-dir-for-tests")
                .join("patterns.db"),
        );
        let error = server
            .list_patterns(Parameters(ListParams { kind: None }))
            .await
            .expect_err("opening a database below a missing directory must fail");

        assert!(!error.message.is_empty());
    }
}
