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
