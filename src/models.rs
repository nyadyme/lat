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
    /// The choice this pattern makes obligatory. Deliberately not unique:
    /// two patterns forcing the same choice carry the same string, which is
    /// what makes it comparable across patterns (`focus` is written to be
    /// distinctive and cannot serve that purpose).
    pub forced_choice: String,
    /// The constituent the pattern interrogates. Closed vocabulary. Together
    /// with `forced_choice` it decides whether two patterns collide: same
    /// choice at the same anchor means their findings are correlated.
    pub attachment: String,
    pub tags: Vec<String>,
    pub themes: Vec<String>,
    /// The work the entry was checked against: author, title, year, or a
    /// reference work where a form has no author. Never empty once the
    /// catalogue is fully researched — an entry nobody has looked up cannot
    /// be told apart from one that was, and that is the whole point of the
    /// field.
    pub source: String,
    /// `sourced` or `contested`. `contested` means checked and defensible,
    /// with the finding disputed in the literature — not "unverified"; an
    /// entry without a source carries the empty value instead and is filtered
    /// out by `exclude_contested`.
    pub status: String,
}

/// The available filter values of a table, so the agent knows valid filters.
#[derive(Debug, Clone, Serialize)]
pub struct Facets {
    pub kind: PatternType,
    pub categories: Vec<String>,
    pub classifications: Vec<String>,
    pub attachments: Vec<String>,
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
    /// Substring within the `forced_choice` field.
    pub forced_choice: Option<String>,
    /// Exact attachment (the constituent a pattern interrogates).
    pub attachment: Option<String>,
    /// Free text across name, description, feature, tags and classification.
    pub text: Option<String>,
    /// Names to exclude from results (e.g. the user's own/source language, so
    /// contrasting lenses surface). Empty means no exclusion.
    pub exclude_names: Vec<String>,
    /// Keep only entries whose `status` is exactly `sourced`. Deliberately
    /// tested that way round rather than as "not contested": an entry with no
    /// source yet carries neither value, and a filter meant to hold back a
    /// disputed finding must not let an unchecked one through instead.
    pub exclude_contested: bool,
}

/// One lens of a combination, as it rides for a given source language.
#[derive(Debug, Clone, Serialize)]
pub struct RosterLens {
    /// Place in the run order, starting at 1. A substitute keeps the place
    /// of the lens it replaces.
    pub position: u32,
    pub name: String,
    pub kind: PatternType,
    /// `opener`, `reduction`, `ablation`, `counter-check` or `reconstruction`.
    pub role: String,
    /// `constructive`, `destructive` or `both`.
    pub polarity: String,
    /// The lens of the language-neutral roster this one stands in for. Absent
    /// on a slot the profile left unchanged.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replaces: Option<String>,
    /// Why the profile substituted the slot. Present exactly when `replaces`
    /// is.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

/// A lens a profile removed without a successor.
#[derive(Debug, Clone, Serialize)]
pub struct DroppedLens {
    pub name: String,
    pub reason: String,
}

/// A combination in its language-neutral form: the trigger stated as a
/// structural fact, and the roster before any profile touched it.
#[derive(Debug, Clone, Serialize)]
pub struct Combination {
    pub id: String,
    pub name: String,
    pub axis: String,
    pub trigger: String,
    pub provenance: String,
    pub roster: Vec<RosterLens>,
}

/// A combination as routed for one source language.
#[derive(Debug, Clone, Serialize)]
pub struct CombinationRoute {
    pub id: String,
    pub name: String,
    pub axis: String,
    /// The language-neutral trigger, so the marker can be read against it.
    pub trigger: String,
    /// `fires`: the language leaves the axis implicit and the combination
    /// applies as written. `inverted`: the language already forces the
    /// choice, so the combination tests whether the forced choice was
    /// warranted. `unrouted`: the database holds no row for this pair — only
    /// reachable through a live edit, since the generator refuses it.
    pub verdict: String,
    /// The surface form that carries the trigger in the source language.
    pub marker: String,
    pub note: String,
    /// The grammar the marker was checked against.
    pub citation: String,
    /// `run`, `derived` or `checked` for this language's row: read off a
    /// run, worked out from a grammar, or tested against real sentences.
    pub provenance: String,
    /// The roster with the profile's substitutions applied.
    pub roster: Vec<RosterLens>,
    /// Lenses the profile removed without a replacement.
    pub dropped: Vec<DroppedLens>,
}

/// The routing of every combination for one source language.
#[derive(Debug, Clone, Serialize)]
pub struct RoutingProfile {
    /// The profile's canonical name, whatever casing was asked for.
    pub source_language: String,
    /// Every catalogue entry describing the source language itself, by exact
    /// name: what goes into `exclude_names` for a text in that language.
    pub own_entries: Vec<String>,
    pub note: String,
    /// Lenses in this profile's rosters that this database's pattern tables
    /// lack, so `get_pattern` will not find them. Only a database seeded by an
    /// older build has any; deleting the file reseeds the catalogue.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub missing_from_catalogue: Vec<String>,
    pub combinations: Vec<CombinationRoute>,
}

/// What exists when no profile was asked for or none matched: the profile
/// names, and the combinations in their language-neutral form.
#[derive(Debug, Clone, Serialize)]
pub struct RoutingOverview {
    /// Set when a requested language has no profile, saying so.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    pub profiles: Vec<String>,
    pub combinations: Vec<Combination>,
}

#[cfg(test)]
#[allow(
    clippy::assert_is_empty,
    reason = "assert!(x.is_empty()) reads as the behavior under test"
)]
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
            forced_choice: "whether two images need a connective".to_owned(),
            attachment: "whole passage".to_owned(),
            tags: vec!["cut".to_owned()],
            themes: vec!["Time & aspect".to_owned()],
            source: "Higginson, The Haiku Handbook".to_owned(),
            status: "sourced".to_owned(),
        };

        let json = serde_json::to_value(&pattern).unwrap();
        assert_eq!(json["kind"], "form");
        assert_eq!(json["name"], "Haiku");
        assert_eq!(json["tags"], serde_json::json!(["cut"]));
        assert_eq!(json["themes"], serde_json::json!(["Time & aspect"]));
        assert_eq!(
            json["forced_choice"],
            "whether two images need a connective"
        );
        assert_eq!(json["attachment"], "whole passage");
        assert_eq!(json["source"], "Higginson, The Haiku Handbook");
        assert_eq!(json["status"], "sourced");
    }

    #[test]
    fn facets_serialize_with_their_kind() {
        let facets = Facets {
            kind: PatternType::Language,
            categories: vec!["Language".to_owned()],
            classifications: vec!["isolate".to_owned()],
            attachments: vec!["subject".to_owned()],
            tags: vec!["ergative".to_owned()],
            themes: vec!["Causality".to_owned()],
        };

        let json = serde_json::to_value(&facets).unwrap();
        assert_eq!(json["kind"], "language");
        assert_eq!(json["categories"], serde_json::json!(["Language"]));
    }

    #[test]
    fn an_unchanged_roster_slot_serializes_without_substitution_fields() {
        let lens = RosterLens {
            position: 1,
            name: "Russian".to_owned(),
            kind: PatternType::Language,
            role: "ablation".to_owned(),
            polarity: "destructive".to_owned(),
            replaces: None,
            reason: None,
        };

        let json = serde_json::to_value(&lens).unwrap();

        assert_eq!(json["kind"], "language");
        assert!(json.get("replaces").is_none());
        assert!(json.get("reason").is_none());
    }

    #[test]
    fn a_substituted_roster_slot_names_what_it_replaces_and_why() {
        let lens = RosterLens {
            position: 1,
            name: "Hindi-Urdu (vector verbs)".to_owned(),
            kind: PatternType::Language,
            role: "ablation".to_owned(),
            polarity: "destructive".to_owned(),
            replaces: Some("Russian".to_owned()),
            reason: Some("own entry".to_owned()),
        };

        let json = serde_json::to_value(&lens).unwrap();

        assert_eq!(json["replaces"], "Russian");
        assert_eq!(json["reason"], "own entry");
    }

    #[test]
    fn an_overview_without_a_message_omits_the_field() {
        let overview = RoutingOverview {
            message: None,
            profiles: vec!["German".to_owned()],
            combinations: Vec::new(),
        };

        let json = serde_json::to_value(&overview).unwrap();

        assert!(json.get("message").is_none());
        assert_eq!(json["profiles"], serde_json::json!(["German"]));
    }

    #[test]
    fn default_filters_are_all_empty() {
        let filters = SearchFilters::default();
        assert!(filters.tag.is_none());
        assert!(filters.theme.is_none());
        assert!(filters.category.is_none());
        assert!(filters.classification.is_none());
        assert!(filters.focus.is_none());
        assert!(filters.forced_choice.is_none());
        assert!(filters.attachment.is_none());
        assert!(filters.text.is_none());
        assert!(filters.exclude_names.is_empty());
        assert!(
            !filters.exclude_contested,
            "the default must not silently narrow a caller's search"
        );
    }
}
