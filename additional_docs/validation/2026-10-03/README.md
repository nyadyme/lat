# Routing check, 2026-10-03

The evidence behind every `checked` row in
[`../../lat_routing.md`](../../lat_routing.md): the markers of K2 to K8, for all
seven profiles, tested against 100 consecutive real sentences per language.

## What was done

The procedure is in [`PROTOCOL.md`](PROTOCOL.md). In short, per language:

1. **Corpus.** 60 sentences from Wikipedia (12 consecutive from the start of
   each of five articles: history, natural science, economics, biography,
   culture) and 40 from freely accessible news and opinion pages, taken in
   order, never chosen for content.
2. **Blind trigger pass.** For each sentence and K2–K8, is the language-neutral
   trigger of the combination present? Written before the profile's markers
   were read.
3. **Marker pass.** Does the profile's marker occur?
4. **Comparison.** `both`, `trigger_only` (the marker misses a real trigger),
   `marker_only` (the marker fires without one), `neither`.
5. **Lens check.** For each substitution, and for lenses the profile notes as
   overlapping, up to 10 trigger sentences: does the lens force a choice the
   source sentence has not already made?

Each language was annotated by one model agent; there was no second rater.

## Results

Cells read `both / trigger_only / marker_only · recall / precision`, where
recall = both / (both + trigger_only) and precision = both / (both + marker_only).
These are the markers **before** revision; the revised markers are in
`lat_routing.md`.

| Language | K2 | K3 | K4 | K5 | K6 | K7 | K8 |
|---|---|---|---|---|---|---|---|
| German | 20/18/26 · 53% / 43% | 22/1/60 · 96% / 27% | 34/8/2 · 81% / 94% | 8/0/11 · 100% / 42% | 47/2/7 · 96% / 87% | 2/16/0 · 11% / 100% | 49/12/3 · 80% / 94% |
| English | 20/27/24 · 43% / 45% | 11/0/70 · 100% / 14% | 12/40/16 · 23% / 43% | 3/13/1 · 19% / 75% | 27/11/29 · 71% / 48% | 2/9/0 · 18% / 100% | 41/10/0 · 80% / 100% |
| French | 8/59/6 · 12% / 57% | 0/10/16 · 0% / 0% | 26/6/3 · 81% / 90% | 7/3/3 · 70% / 70% | 37/1/33 · 97% / 53% | 2/12/0 · 14% / 100% | 39/5/12 · 89% / 76% |
| Spanish | 28/27/4 · 51% / 88% | 5/12/9 · 29% / 36% | 22/5/37 · 81% / 37% | 10/0/17 · 100% / 37% | 42/2/34 · 95% / 55% | 7/15/0 · 32% / 100% | 26/8/14 · 76% / 65% |
| Russian | 31/26/0 · 54% / 100% | 23/9/31 · 72% / 43% | 13/24/8 · 35% / 62% | 11/0/4 · 100% / 73% | 30/6/49 · 83% / 38% | 7/9/0 · 44% / 100% | 54/24/9 · 69% / 86% |
| Polish | 7/34/6 · 17% / 54% | 3/1/11 · 75% / 21% | 11/17/11 · 39% / 50% | 1/4/5 · 20% / 17% | 20/5/48 · 80% / 29% | 4/10/1 · 29% / 80% | 41/27/17 · 60% / 71% |
| Japanese | 46/14/12 · 77% / 79% | 44/0/50 · 100% / 47% | 34/3/22 · 92% / 61% | 13/1/50 · 93% / 21% | 30/1/38 · 97% / 44% | 0/5/4 · 0% / 0% | 78/4/4 · 95% / 95% |

The counts were recomputed from `triggers.json` and `markers.json` and agree
with the figures each agent reported.

What the check changed:

- All 49 K2–K8 markers were revised.
- **Verdicts:** all held, including K5 `inverted` for Russian and Polish.
- **Japanese K4:**
  - Yucatec is now replaced by Hausa instead of Spanish (differential object marking).
  - The Hungarian → Hausa substitution was withdrawn.
- **Russian and Polish K5:** Finnish (partitive object) is dropped.
- **French K5:** Russian is replaced by Hindi-Urdu (vector verbs); the past
  already forces the perfective choice, and Russian was redundant in 9 of 10.
- **K8, all profiles:** the marker is no longer restricted to argument slots.
- **German and Spanish K8:** Ancient Greek (article & substantivization) is
  dropped; both grammars share the device (10 of 10 and 7 of 10).

The per-language detail is in each folder's `report.md`.

## Layout

One folder per language:

| File | Content |
|---|---|
| `manifest.json` | Every sentence: `id`, `domain`, `kind` (`wikipedia` / `news`), `url`, `position_in_run`, `sha256` of the NFC text. Wikipedia rows also carry `revid`, `revision_timestamp` and `found_in_revision` |
| `wikipedia_text.json` | Text and English gloss of the 60 Wikipedia sentences (Japanese also romanization), CC BY-SA 4.0 — see [`ATTRIBUTION.md`](ATTRIBUTION.md) |
| `triggers.json` | Blind pass: per sentence and K2–K8, `present` and `span` |
| `markers.json` | Marker pass, same shape |
| `counts.json` | Recomputed counts with the sentence ids in each cell |
| `lens_check.json` | Lens checks where the agent wrote them as data (Spanish, Russian, Polish); elsewhere they are in `report.md` |
| `report.md` | The agent's report |
| `triggers_revised.json` | German only: seven trigger cells corrected after the marker pass for consistency. The counts use the blind file |

## What is not here, and why

- **News text.** The 40 news sentences per language come from copyrighted
  articles, some taken in full (Polsat News, two franceinfo articles). Only
  their URL, position and SHA-256 are kept, and annotation spans on them are
  cut to at most 40 characters and at most half the sentence, so the spans
  together cannot rebuild an article. To recover a sentence: fetch the URL,
  split the body into sentences, take the run in order, and compare the
  SHA-256 of the NFC-normalised text. Pages behind a paywall or edited since
  will not match.
- **Fetched HTML, scripts and per-miss files.** The raw pages contain the full
  news articles. The Spanish `cases.json` and Russian `misses.json` quoted
  whole sentences; their findings are summarised in the reports.

## Caveats

- **One annotator per language.** K3 (inference against record), K4 (what
  counts as a bundle) and K5 (what counts as a running process) are judgement
  calls, and several agents report their own blind-pass misses. Read the
  numbers as direction, not measurement. The effects acted on were large:
  K3 precision 0–47 %, K7 recall 0–44 %, English K4 bare plural 0 of 52.
- **Reconstructed reports.** The Spanish, Russian, Polish and Japanese agents
  were not permitted to write report files. Their `report.md` was
  reconstructed from the agent's final message.
- **No Japanese lens-check data.** Japanese has no `lens_check.json`; its lens
  counts exist only in its report.
- **One count corrected.** The English report gives Finnish (partitive
  object) in K5 as forcing a new choice in 10 of 10. A recount of the English
  K5 trigger sentences shows that only about 5 of 16 have a direct object for
  the lens to attach to. Most are intransitive change verbs (*expanded*,
  *emerged*, *developed*), so the 10 of 10 overstates it.
- **Wikipedia revisions found afterwards.** The agents did not record revision
  ids. Each `revid` is the revision current at 2026-10-03 23:59 UTC, looked up
  afterwards. 296 of the 300 Wikipedia sentences were found in that revision.
  The other four (English 48, Japanese 26, 59, 60) are marked
  `found_in_revision: false`; they may have been fetched from a slightly
  different revision or differ only in quotation formatting.

## Licence

The Wikipedia sentences in `wikipedia_text.json` are CC BY-SA 4.0; see
[`ATTRIBUTION.md`](ATTRIBUTION.md). Everything else in this folder — the
annotations, counts, manifests and reports — is part of this repository and
falls under the same terms as `lat_routing.md`.
