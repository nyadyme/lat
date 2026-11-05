# Routing check, 2026-11-05

This folder holds the evidence behind the `checked` rows of the **Chinese**,
**Arabic**, **Hindi** and **Turkish** profiles in
[`../../lat_routing.md`](../../lat_routing.md).
The markers of K2 to K8 were tested against 100 consecutive real sentences per
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
  - The BBC Türkçe report supplies 25 and Euronews Türkçe 15; DW Türkçe had
    only 11.
  - In the Arabic, Hindi and Turkish pages, pull quotes, teasers and captions
    were removed by rule: any paragraph without closing punctuation was
    dropped.
- **Wikipedia revisions.** Each Wikipedia revision id was recorded at fetch
  time, so `found_in_revision` is true for every row.
- **Lens check.** The lens check is in `lens_check.json` in each language
  folder.

All four profiles were written by a different model session before the check.
Each annotating agent read only the Combinations table until it had written
`triggers.json`.

## Results

Cells read `both / trigger_only / marker_only · recall / precision`. They show
the markers before revision.

| Language | K2 | K3 | K4 | K5 | K6 | K7 | K8 |
|---|---|---|---|---|---|---|---|
| Chinese | 65/8/1 · 89% / 98% | 56/0/34 · 100% / 62% | 43/16/1 · 73% / 98% | 10/8/21 · 56% / 32% | 31/13/5 · 70% / 86% | 13/6/4 · 68% / 76% | 85/3/5 · 97% / 94% |
| Arabic | 62/18/1 · 78% / 98% | 42/0/27 · 100% / 61% | 39/32/4 · 55% / 91% | 31/0/35 · 100% / 47% | 63/0/5 · 100% / 93% | 7/18/1 · 28% / 88% | 85/2/7 · 98% / 92% |
| Hindi | 44/13/9 · 77% / 83% | 54/0/36 · 100% / 60% | 26/10/14 · 72% / 65% | 20/1/32 · 95% / 38% | 45/0/35 · 100% / 56% | 8/3/2 · 73% / 80% | 35/0/43 · 100% / 45% |
| Turkish | 63/8/3 · 89% / 95% | 27/22/15 · 55% / 64% | 41/8/20 · 84% / 67% | 25/4/16 · 86% / 61% | 47/2/26 · 96% / 64% | 6/7/3 · 46% / 67% | 73/0/8 · 100% / 90% |

The counts were recomputed from `triggers.json` and `markers.json`. They agree
with the agents' figures.

What the check changed:

- **Markers.** All K2–K8 markers of the four profiles were revised; the
  details are in each folder's `report.md`.
- **Verdicts.** All verdicts held, the `inverted` Hindi K5 included (aspect was
  forced in 20 of 21 trigger sentences).
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
- **Hindi K5.**
  - The derived replacement of Russian by Luganda (graded tense) was
    withdrawn: Russian still forced a new choice in 6 of 10 sentences,
    because the Hindi perfective does not entail that the goal was reached.
- **Hindi K6.** Mongolian stays dropped: 0 of 10.
- **Hindi K8.** Ancient Greek is now dropped: 0 of 10.
- **Turkish K3.** The verdict stays `fires`: the past evidential *-mIş*
  settled none of 49 trigger sentences, and Tuyuca forced a new choice in 10
  of 10.
- **Turkish K5.** Russian stays replaced by Hindi-Urdu (vector verbs): 3 of
  10 against 9 of 10.
- **Turkish K8.** Ancient Greek is now dropped: 0 of 10.

## Licence

The Wikipedia sentences in the `wikipedia_text.json` of each language folder
are under CC BY-SA 4.0; see
[`ATTRIBUTION.md`](ATTRIBUTION.md). Everything else here falls under the same
terms as `lat_routing.md`.
