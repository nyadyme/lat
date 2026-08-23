// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 Dana Schlifka

//! Data types for the thinking patterns (languages and forms).

use rmcp::schemars;
use serde::{Deserialize, Serialize};

/// Kind of a pattern. Determines which table is read.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum PatternType {
    /// A bound poetic form or writing technique.
    Form,
    /// A natural language or linguistic register.
    Language,
}

impl PatternType {
    /// Name of the SQLite table for this pattern kind.
    pub fn table(self) -> &'static str {
        match self {
            PatternType::Form => "forms",
            PatternType::Language => "languages",
        }
    }
}

/// A complete pattern with all fields.
#[derive(Debug, Clone, Serialize)]
pub struct Pattern {
    pub kind: PatternType,
    pub name: String,
    pub description: String,
    pub focus: String,
    pub category: String,
    pub classification: String,
    pub feature: String,
    pub tags: Vec<String>,
    pub themes: Vec<String>,
}

/// The available filter values of a table, so the agent knows valid filters.
#[derive(Debug, Clone, Serialize)]
pub struct Facets {
    pub kind: PatternType,
    pub categories: Vec<String>,
    pub classifications: Vec<String>,
    pub tags: Vec<String>,
    pub themes: Vec<String>,
}

/// Filter criteria for a search. All optional, combined with AND.
#[derive(Debug, Clone, Default)]
pub struct SearchFilters {
    /// Exact tag (an element of the tags array).
    pub tag: Option<String>,
    /// Exact theme (an element of the themes array).
    pub theme: Option<String>,
    /// Exact category.
    pub category: Option<String>,
    /// Exact classification.
    pub classification: Option<String>,
    /// Substring within the focus field.
    pub focus: Option<String>,
    /// Free text across name, description, feature, tags and classification.
    pub text: Option<String>,
    /// Names to exclude from results (e.g. the user's own/source language, so
    /// contrasting lenses surface). Empty means no exclusion.
    pub exclude_names: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn each_kind_names_its_own_table() {
        assert_eq!(PatternType::Form.table(), "forms");
        assert_eq!(PatternType::Language.table(), "languages");
    }

    #[test]
    fn a_kind_is_lowercase_on_the_wire() {
        // The MCP tool schema advertises 'form' and 'language'; hosts send
        // exactly those strings.
        assert_eq!(
            serde_json::to_value(PatternType::Form).unwrap(),
            serde_json::json!("form")
        );
        assert_eq!(
            serde_json::to_value(PatternType::Language).unwrap(),
            serde_json::json!("language")
        );
    }

    #[test]
    fn a_kind_is_read_back_from_its_lowercase_name() {
        let form: PatternType = serde_json::from_str("\"form\"").unwrap();
        assert_eq!(form, PatternType::Form);

        let language: PatternType = serde_json::from_str("\"language\"").unwrap();
        assert_eq!(language, PatternType::Language);
    }

    #[test]
    fn an_unknown_kind_is_rejected() {
        assert!(serde_json::from_str::<PatternType>("\"Form\"").is_err());
        assert!(serde_json::from_str::<PatternType>("\"dialect\"").is_err());
    }

    #[test]
    fn a_pattern_serializes_with_its_kind_and_arrays() {
        let pattern = Pattern {
            kind: PatternType::Form,
            name: "Haiku".to_owned(),
            description: "a cut".to_owned(),
            focus: "brevity".to_owned(),
            category: "Poetic form".to_owned(),
            classification: "Japanese".to_owned(),
            feature: "seventeen morae".to_owned(),
            tags: vec!["cut".to_owned()],
            themes: vec!["Time & aspect".to_owned()],
        };

        let json = serde_json::to_value(&pattern).unwrap();
        assert_eq!(json["kind"], "form");
        assert_eq!(json["name"], "Haiku");
        assert_eq!(json["tags"], serde_json::json!(["cut"]));
        assert_eq!(json["themes"], serde_json::json!(["Time & aspect"]));
    }

    #[test]
    fn facets_serialize_with_their_kind() {
        let facets = Facets {
            kind: PatternType::Language,
            categories: vec!["Language".to_owned()],
            classifications: vec!["isolate".to_owned()],
            tags: vec!["ergative".to_owned()],
            themes: vec!["Causality".to_owned()],
        };

        let json = serde_json::to_value(&facets).unwrap();
        assert_eq!(json["kind"], "language");
        assert_eq!(json["categories"], serde_json::json!(["Language"]));
    }

    #[test]
    fn default_filters_are_all_empty() {
        let filters = SearchFilters::default();
        assert!(filters.tag.is_none());
        assert!(filters.theme.is_none());
        assert!(filters.category.is_none());
        assert!(filters.classification.is_none());
        assert!(filters.focus.is_none());
        assert!(filters.text.is_none());
        assert!(filters.exclude_names.is_empty());
    }
}
