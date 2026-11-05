# Routing check, 2026-11-05

This folder holds the evidence behind the `checked` rows of the **Chinese** and
**Arabic** profiles in [`../../lat_routing.md`](../../lat_routing.md). The
markers of K2 to K8 were tested against 100 consecutive real sentences per
language.

## Method

The procedure, file layout, copyright handling of news text and caveats are
those of the [2026-10-03 check](../2026-10-03/README.md). The protocol is
[`../2026-10-03/PROTOCOL.md`](../2026-10-03/PROTOCOL.md).

This check differed from the earlier one in four ways:

- **The corpus was built before annotation began**, by a script, so the
  sentences were not chosen by the annotator. Each article was fetched through
  the MediaWiki API, the Chinese ones as their zh-cn variant.
- **News text.** The two news pieces per language were the most recent on
  their index pages.
  - The Xinhua body has 19 sentences, so DW supplies 21.
  - In the Arabic pages, pull quotes, teasers and captions were removed by
    rule: any paragraph without closing punctuation was dropped.
- **Wikipedia revisions.** Each Wikipedia revision id was recorded at fetch
  time, so `found_in_revision` is true for every row.
- **Lens check.** The lens check is in `lens_check.json` in each language
  folder.

Both profiles were written by a different model session before the check.
Each annotating agent read only the Combinations table until it had written
`triggers.json`.

## Results

Cells read `both / trigger_only / marker_only · recall / precision`. They show
the markers before revision.

| Language | K2 | K3 | K4 | K5 | K6 | K7 | K8 |
|---|---|---|---|---|---|---|---|
| Chinese | 65/8/1 · 89% / 98% | 56/0/34 · 100% / 62% | 43/16/1 · 73% / 98% | 10/8/21 · 56% / 32% | 31/13/5 · 70% / 86% | 13/6/4 · 68% / 76% | 85/3/5 · 97% / 94% |
| Arabic | 62/18/1 · 78% / 98% | 42/0/27 · 100% / 61% | 39/32/4 · 55% / 91% | 31/0/35 · 100% / 47% | 63/0/5 · 100% / 93% | 7/18/1 · 28% / 88% | 85/2/7 · 98% / 92% |

The counts were recomputed from `triggers.json` and `markers.json`. They agree
with the agents' figures.

What the check changed:

- **Markers.** All seven Chinese and all seven Arabic K2–K8 markers were
  revised; the details are in each folder's `report.md`.
- **Verdicts.** All `fires` verdicts held in both profiles.
- **Chinese K4.** Yucatec Maya is still replaced by Hausa (pluractional
  verbs). Yucatec forced a new choice in 0 of 10 sentences, Hausa in 4 of 10.
- **Chinese K7.** Classical Chinese (Wényán) stays; it forced a new choice in 10 of 10.
- **Arabic K5.**
  - Russian is still replaced by Hindi-Urdu (vector verbs): Russian forced a
    new choice in 2 of 10 sentences, Hindi-Urdu in 7 of 10.
  - Finnish (partitive object) is now dropped: 0 of 8 sentences with an
    object.
- **Arabic K8.** Ancient Greek (article & substantivization) stays dropped:
  0 of 10.

## Licence

The Wikipedia sentences in `chinese/wikipedia_text.json` and
`arabic/wikipedia_text.json` are under CC BY-SA 4.0; see
[`ATTRIBUTION.md`](ATTRIBUTION.md). Everything else here falls under the same
terms as `lat_routing.md`.
