# Validation protocol: routing profiles on 100 sentences

Repo: /Users/dana/dev/lat (READ ONLY — never modify repo files).
Data under test: additional_docs/lat_routing.md (sections Combinations, Rosters,
Profiles, Routing, Substitutions). Lens details: additional_docs/lat_catalog.md.
Write everything to: <SCRATCH>/validation/<LANG>/  (SCRATCH given in your prompt).

## Step 1 — Corpus (100 real sentences, no cherry-picking)
- 60 sentences from Wikipedia in the target language: 5 articles from 5 distinct
  domains (history, natural science, politics/economics, biography, culture/
  society), 12 CONSECUTIVE sentences from each, starting at the first sentence
  of the article body (skip infobox/lead fragments that are not full sentences).
- 40 sentences from freely accessible news or opinion/commentary pages in the
  target language (at least 2 different outlets/articles), again consecutive
  runs, taken in order.
- Never pick or skip sentences for their content. Record the URL per sentence.
- Save as corpus.json: [{"id": 1, "url": "...", "domain": "...", "text": "...",
  "en": "<literal English gloss>"}].
- If a page cannot be fetched, take another page of the same domain; note it.

## Step 2 — Blind trigger pass (do NOT read the Routing/Substitutions sections yet)
Read only the "Combinations" table (language-neutral triggers). For every
sentence and every combination K2–K8 decide: trigger present? (bool) plus the
span. K1 is always present. Be strict and literal about the trigger wording.
Save triggers.json: {"<id>": {"K2": {"present": true, "span": "..."}, ...}}.
Write this file BEFORE step 3.

## Step 3 — Marker pass
Now read the target language's rows in the Routing table. For every sentence
and K2–K8: does the profile's marker (the surface form it names) occur? (bool,
span). Save markers.json in the same shape.

## Step 4 — Comparison
Per combination compute: both, trigger_only (profile marker misses a real
trigger), marker_only (marker fires without the trigger), neither. Use a small
python3 script; save counts.json. For each trigger_only and marker_only case
keep the sentence id and a one-line reason. Group the misses into patterns:
what surface form actually carried the trigger when the marker missed it?

## Step 5 — Verdict check
For each combination: does the verdict (fires / inverted) match what the
sentences show? For an `inverted` row: in sentences with the trigger, did the
grammar indeed already force the choice (e.g. aspect)? For a `fires` row whose
note says the language partly encodes the axis: how often was it encoded?

## Step 6 — Lens check (substitutions and noted overlaps)
For each Substitutions row of this language, and for each roster lens the
language's Routing notes say partly overlaps the source grammar, take up to 10
sentences where that combination's trigger is present. For each: (a) would the
removed / questioned lens force a choice the source sentence has NOT already
made grammatically? (b) does the replacement (if any) force one? Count a/b.
Also, without being asked by the profile: if any OTHER roster lens of a firing
combination proves redundant against this language's grammar in >= 7 of 10
sentences, report it.

## Step 7 — Report
Save report.md and return a summary (max 450 words) with:
- corpus composition (domains, URLs count), anything that deviated from Step 1
- a table K2–K8: both / trigger_only / marker_only / neither
- each proposed change to the profile, concretely: row, current text, proposed
  text, evidence (sentence ids, counts). Distinguish "clear" (supported by
  >= 3 sentences or a count) from "tentative".
- substitution / lens verdicts with counts
- your own doubts: where annotation was a judgement call.
Be honest: if the profile holds, say so; do not invent problems.
