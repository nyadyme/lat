// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 Dana Schlifka

//! Regression tests for the collision screen of the catalogue.
//!
//! Two patterns collide when they force the **same choice** at the **same
//! attachment point**: their findings are then correlated, and a matrix that
//! lists both presents one piece of evidence as two. The screen is a
//! comparison of two seed columns, so it can be checked mechanically — but
//! only the *data* can be checked here. Whether a collision group is also
//! documented as an anti-combination lives in the skill prose, and nothing in
//! this crate can enforce that. What this file does instead is freeze the set
//! of collision groups, so that adding a pattern or editing a `forced_choice`
//! string fails loudly and forces the prose to be revisited.
//!
//! The seed is parsed directly rather than going through a running server:
//! the invariants are properties of the catalogue, and the end-to-end exposure
//! of both columns is already covered by `mcp_stdio.rs`.
//!
//! ## The Basque/Burushaski group
//!
//! `Basque (Euskara)` and `Burushaski` share
//! (`whether the act was willed`, `subject`) and are therefore listed as an
//! anti-combination. The pairing was challenged on the grounds that Basque
//! asks whether a causer is named at all while Burushaski grades intention.
//! It was tested against 130 hand-scored German sentences — an exploratory
//! batch of 30 and the adversarial batch of 100 kept in
//! `fixtures/agency_probe.tsv`, built specifically to separate the two.
//!
//! Result: read the way the `forced_choice` string defines it, Basque never
//! parts from Burushaski — not in one sentence out of 130. The collision is
//! real and the anti-combination is load-bearing.
//!
//! The same corpus did find a split, but inside the Basque entry rather than
//! between the two lenses: its `forced_choice` string asks about the **act**,
//! its `feature` text about the **participant**, and on the class "deliberate
//! agent, unintended act" (*Der Chirurg durchtrennte die Arterie*) those give
//! opposite answers — 25 of 100 sentences. The screen reads the string, so the
//! machinery is correct; a human deciding whether to trust the pairing reads
//! the feature text, which points the other way. That divergence is frozen
//! below too, so that rewording either field is a deliberate act.

use std::collections::{BTreeMap, BTreeSet};

/// The catalogue, as applied on first start.
const SEED: &str = include_str!("../src/seed.sql");

/// The probe corpus behind the Basque/Burushaski group.
const PROBE: &str = include_str!("fixtures/agency_probe.tsv");

/// Column offsets of a seed row, in the order the INSERT lists them.
const NAME: usize = 0;
const FORCED_CHOICE: usize = 6;
const ATTACHMENT: usize = 7;

/// Every column a seed row must carry.
const COLUMNS: usize = 10;

/// The constituents a pattern may interrogate. Closed on purpose: the anchor
/// is half of the collision screen, so a free-text anchor would let two
/// patterns miss each other by wording alone.
const ANCHORS: [&str; 11] = [
    "connective",
    "noun",
    "object",
    "person",
    "possessive",
    "spatial frame",
    "subject",
    "surface",
    "verb",
    "whole passage",
    "word order",
];

/// Every group of patterns sharing an (attachment, forced_choice) pair.
///
/// Three of these are documented as anti-combinations in the skill prose
/// (Basque/Burushaski, Nez Percé/Oromo, Tuyuca/Tariana). The other three are
/// not, and one of them — Guugu Yimithirr with Tzeltal — is named as a
/// combination in the routing table for `Space & orientation`, the row the
/// prose itself flags as "deliberately not a built-out combination".
const KNOWN_COLLISIONS: [(&str, &str, &[&str]); 6] = [
    (
        "spatial frame",
        "whether space is reckoned from the speaker or from the world",
        &["Guugu Yimithirr", "Tzeltal (uphill-downhill axis)"],
    ),
    (
        "subject",
        "whether the act was willed",
        &["Basque (Euskara)", "Burushaski"],
    ),
    (
        "subject",
        "whether the participant is the one acting",
        &["Nez Percé (Nimipuutímt)", "Oromo (marked nominative)"],
    ),
    (
        "verb",
        "the physical shape of the thing handled",
        &["Cherokee (Tsalagi)", "Navajo (shape verbs & evidence)"],
    ),
    (
        "verb",
        "the source of the information",
        &["Tariana", "Tuyuca"],
    ),
    (
        "whole passage",
        "which two lines the thought keeps returning to",
        &["Triolet", "Villanelle"],
    ),
];

/// One catalogue entry, reduced to what the screen compares.
struct Entry {
    name: String,
    forced_choice: String,
    attachment: String,
}

/// Splits a seed row into its single-quoted fields, honouring `''` escapes.
fn quoted_fields(line: &str) -> Vec<String> {
    let chars: Vec<char> = line.chars().collect();
    let mut fields = Vec::new();
    let mut i = 0;

    while i < chars.len() {
        if chars[i] != '\'' {
            i += 1;
            continue;
        }
        i += 1;
        let mut field = String::new();
        while i < chars.len() {
            if chars[i] == '\'' {
                if chars.get(i + 1) == Some(&'\'') {
                    field.push('\'');
                    i += 2;
                    continue;
                }
                i += 1;
                break;
            }
            field.push(chars[i]);
            i += 1;
        }
        fields.push(field);
    }
    fields
}

/// Reads every pattern out of the seed.
fn catalogue() -> Vec<Entry> {
    SEED.lines()
        .map(str::trim)
        .filter(|line| line.starts_with("('"))
        .map(|line| {
            let fields = quoted_fields(line);
            assert!(
                fields.len() >= COLUMNS,
                "seed row has {} fields, expected at least {COLUMNS}: {line}",
                fields.len()
            );
            Entry {
                name: fields[NAME].clone(),
                forced_choice: fields[FORCED_CHOICE].clone(),
                attachment: fields[ATTACHMENT].clone(),
            }
        })
        .collect()
}

/// Groups the catalogue by the pair the screen compares.
fn collisions() -> BTreeMap<(String, String), Vec<String>> {
    let mut groups: BTreeMap<(String, String), Vec<String>> = BTreeMap::new();
    for entry in catalogue() {
        groups
            .entry((entry.attachment, entry.forced_choice))
            .or_default()
            .push(entry.name);
    }
    groups.retain(|_, members| members.len() > 1);
    for members in groups.values_mut() {
        members.sort();
    }
    groups
}

#[test]
fn the_catalogue_is_not_empty() {
    assert_eq!(
        catalogue().len(),
        130,
        "pattern count changed; the frozen collision set below has to be re-derived"
    );
}

#[test]
fn every_pattern_declares_a_forced_choice_and_an_attachment() {
    // A database written by an older build gets both columns added empty by
    // the migration and is never reseeded, because seeding only runs on an
    // empty table. Empty cells then look exactly like absent fields, and the
    // screen silently matches nothing. Guard the source of truth at least.
    for entry in catalogue() {
        assert!(
            !entry.forced_choice.trim().is_empty(),
            "{} has no forced_choice",
            entry.name
        );
        assert!(
            !entry.attachment.trim().is_empty(),
            "{} has no attachment",
            entry.name
        );
    }
}

#[test]
fn attachment_uses_the_closed_vocabulary_of_anchors() {
    let allowed: BTreeSet<&str> = ANCHORS.into_iter().collect();
    let used: BTreeSet<String> = catalogue().into_iter().map(|e| e.attachment).collect();

    for anchor in &used {
        assert!(
            allowed.contains(anchor.as_str()),
            "unknown attachment {anchor:?}; the anchor list is closed on purpose"
        );
    }
    assert_eq!(
        used.len(),
        ANCHORS.len(),
        "an anchor fell out of use: {:?}",
        allowed
            .iter()
            .filter(|a| !used.contains(**a))
            .collect::<Vec<_>>()
    );
}

#[test]
fn the_catalogue_has_exactly_the_known_collision_groups() {
    let found = collisions();
    let expected: BTreeMap<(String, String), Vec<String>> = KNOWN_COLLISIONS
        .iter()
        .map(|(attachment, choice, members)| {
            (
                ((*attachment).to_owned(), (*choice).to_owned()),
                members.iter().map(|m| (*m).to_owned()).collect(),
            )
        })
        .collect();

    assert_eq!(
        found, expected,
        "the set of colliding patterns changed. Every group here is a pair \
         whose findings are correlated, so the anti-combination list in the \
         skill prose has to be updated along with this constant."
    );
}

#[test]
fn basque_and_burushaski_still_force_the_same_choice() {
    // Singled out because this pairing was challenged and then upheld against
    // 130 sentences; see the module documentation.
    let groups = collisions();
    let members = groups
        .get(&(
            "subject".to_owned(),
            "whether the act was willed".to_owned(),
        ))
        .expect("Basque and Burushaski no longer share an anchor and a choice");

    assert_eq!(members, &["Basque (Euskara)", "Burushaski"]);
}

/// One scored sentence of the probe corpus.
struct Probe {
    group: String,
    basque_act: String,
    basque_agent: String,
    burushaski: String,
    sentence: String,
}

/// Reads the corpus, skipping the comment header.
fn probe() -> Vec<Probe> {
    PROBE
        .lines()
        .filter(|line| !line.starts_with('#') && !line.is_empty())
        .skip(1) // column header
        .map(|line| {
            let cell: Vec<&str> = line.split('\t').collect();
            assert_eq!(cell.len(), 6, "malformed probe row: {line}");
            Probe {
                group: cell[0].to_owned(),
                basque_act: cell[1].to_owned(),
                basque_agent: cell[2].to_owned(),
                burushaski: cell[3].to_owned(),
                sentence: cell[5].to_owned(),
            }
        })
        .collect()
}

/// Whether a verdict amounts to "there is a willed agent here".
fn sees_a_willed_agent(verdict: &str) -> bool {
    match verdict {
        "ERG" | "WILLED" => true,
        "ABS" | "NONE" | "UNWILLED" | "NOACT" => false,
        other => panic!("unknown verdict {other:?}"),
    }
}

#[test]
fn the_probe_corpus_is_well_formed() {
    let rows = probe();
    assert_eq!(rows.len(), 100);
    assert!(
        rows.iter().filter(|r| r.group == "A").count() == 20,
        "the adversarial class 'deliberate agent, unintended act' must stay at \
         20 sentences; it is the only class that separates the two readings"
    );
}

#[test]
fn the_act_scoped_reading_of_basque_never_parts_from_burushaski() {
    let divergent: Vec<String> = probe()
        .into_iter()
        .filter(|r| sees_a_willed_agent(&r.basque_act) != sees_a_willed_agent(&r.burushaski))
        .map(|r| r.sentence)
        .collect();

    assert!(
        divergent.is_empty(),
        "Basque and Burushaski were expected to agree on every sentence, \
         but parted on: {divergent:?}"
    );
}

#[test]
fn the_two_readings_of_the_basque_entry_part_on_the_unintended_act_class() {
    let rows = probe();
    let divergent: Vec<&Probe> = rows
        .iter()
        .filter(|r| sees_a_willed_agent(&r.basque_act) != sees_a_willed_agent(&r.basque_agent))
        .collect();

    assert_eq!(
        divergent.len(),
        25,
        "the split between the forced_choice string and the feature text \
         changed size; if either field was reworded, re-derive this number"
    );
    for row in &divergent {
        assert!(
            matches!(row.group.as_str(), "A" | "C" | "D" | "E"),
            "the split was confined to acts that miscarried, but {:?} is in \
             group {}",
            row.sentence,
            row.group
        );
    }
}
