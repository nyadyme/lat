# Turkish profile: validation report

*The annotating agent was not permitted to write report files. This report was
reconstructed from its final message. Sentence ids refer to `manifest.json`.*

There was one annotator, one pass and no second rater.

All `fires` verdicts hold. That includes K3, where the past evidential settled none of
the triggers. The weak points were the marker texts:
- K3 missed the *-mIştIr* and *-mIştI* forms.
- K5 and K6 fired on verbal nouns and participles.
- K7 missed several causal postpositions.

## Corpus (7 URLs)

The corpus has 100 sentences:
- **Wikipedia, 60 sentences.** 12 consecutive sentences from the start of each of five
  articles: Malazgirt Meydan Muharebesi (history), Fotosentez (science),
  Enflasyon (economics), Nâzım Hikmet (biography) and Türk kahvesi (culture).
- **News, 40 sentences:**
  - a whole BBC Türkçe report (ids 61–85, 25 sentences);
  - the first 15 sentences of a Euronews Türkçe report (ids 86–100).

  Both are of 2026-11-05.

Deviations:
- The corpus was built before annotation began, by a separate script.
- T24 and Bianet load their article lists with JavaScript and could not be
  fetched.
- DW Türkçe was tried first, but its latest report has only 11 sentences, so
  the two reports split 25 / 15, not 20 / 20.
- Captions, the BBC page footer and the DW standfirst were removed by rule.
- Ids 11–12 are one sentence split at "IX."; 11 has no finite verb.
- A two-sentence quotation spans 77–78.
- The Euronews report concerns a named person under investigation. It was
  annotated strictly as language data, and the glosses write the name as
  [named person].

## Counts

These are the markers **before** revision.

| | both | trigger_only | marker_only | neither | recall | precision |
|---|---|---|---|---|---|---|
| K2 | 63 | 8 | 3 | 26 | 89% | 95% |
| K3 | 27 | 22 | 15 | 36 | 55% | 64% |
| K4 | 41 | 8 | 20 | 31 | 84% | 67% |
| K5 | 25 | 4 | 16 | 55 | 86% | 61% |
| K6 | 47 | 2 | 26 | 25 | 96% | 64% |
| K7 | 6 | 7 | 3 | 84 | 46% | 67% |
| K8 | 73 | 0 | 8 | 19 | 100% | 90% |

## K3 verdict check

There were 49 trigger sentences:
- 19 assert direct with *-DI*.
- 20 contain *-mIş* in form: 12 the certainty perfect *-mIştIr* or a
  suspended *-mIş*, and 8 the pluperfect *-mIştI*.
- 0 carry an evidential *-mIş* or *-(y)mIş*.
- 10 sit where the choice is not forced: 8 present or copula, 1 future, 1
  fragment.

The past evidential settled none of the triggers, so the verdict stays `fires`,
not `inverted`.

## Changes applied to `lat_routing.md`

**Clear:**
- **K3.** The marker now names the *-DI* past, the present or future, the
  certainty perfect *-mIştIr* and the pluperfect *-mIştI*. Before, it covered
  only the *-DI* past and the present, and 20 of the 22 trigger_only cases were
  these forms (*-mIştIr*: 2, 3, 15, 38, 40–45, 56, 57; *-mIştI*: 7, 8, 72, 84,
  90, 97–99).
- **K5.**
  - Added participles *-(y)An* / *-DIk* over a process (1, 52, 76, 86).
  - Restricted the verbal noun to a past process. 13 of the 16 marker_only
    cases were generic or future verbal nouns.
- **K6.** Added a note that genitive and possessive agreement on a verbal noun
  or participle, partitives and postpositions are not possession. They made up
  22 of the 26 marker_only cases.
- **K7.** Added *sonucu*, *sonuç olarak*, *gereği*, *üzerine* and causal
  *-(y)lA* (22, 23, 27, 45, 56, 66, 96).
- **K4.** Added plural ethnonyms and collectives acting as one bloc
  (*Türkler*, *Bizanslılar*: 2, 4, 6, 7, 10).

**Tentative, kept as notes:**
- K3: named sources such as *görüşünde* were added to the list of forms that
  already mark a report. 15 marker_only cases were named sources, and the
  annotator did not count them as triggers.
- K4: a generic mass and an institution not acting as one unit made up most of
  the 20 false hits.
- K5: inceptive *-mAyA başla-* (12, 66).
- K6: *kendi* as a plain reflexive (31, 85, 94).
- K2: a state or institution holding a view (*AB … görüşünde*: 61, 62, 73, 76),
  and a non-actor named with *tarafından* (10, 67).

**Substitutions:**
- **K5: Russian → Hindi-Urdu (vector verbs) holds.** Russian forced no new
  choice in 7 of 10 sentences, and all three exceptions were non-finite.
  Hindi-Urdu forced one in 9 of 10.
- **K8: Ancient Greek (article & substantivization) is newly dropped.** It
  forced a new choice in 0 of 10, because overt derivation already shows that
  something is being made a noun.

**Not applied:** none. The K8 marker holds; its 8 marker_only cases are
relational adjectives, which the row includes on purpose.

## Lens check

The sample is 10 trigger ids evenly spaced over each combination. "Forces"
means the lens forces a choice the Turkish sentence has not already made. The
per-sentence data is in `lens_check.json`.

| Comb. | Lens | Forces |
|---|---|---|
| K3 | Tuyuca | 10 of 10; *-DI* / *-mIş* gives at most direct against indirect, never report against inference |
| K3 | Ewe (logophoric pronouns) | not applicable in all 10 |
| K3 | Latin (oratio obliqua) | 6 of 10 |
| K5 | Russian | 3 of 10; all three exceptions non-finite |
| K5 | Hindi-Urdu (vector verbs) | 9 of 10 |
| K5 | Finnish (partitive object) | 2 of 5 (5 n/a) |
| K5 | Guaraní | 10 of 10 |
| K4 | Yucatec Maya | 8 of 10; most triggers are state names, which are not number-neutral |
| K4 | Polish (numeral threshold) | 1 of 1 in the sample; 4 of 4 across all numeral or quantifier triggers |
| K4 | Hungarian (associative plural) | 6 of 7; associative *-lAr* never occurs |
| K2 | Basque (Euskara) | 4 of 10 |
| K2 | Nez Percé (Nimipuutímt) | 4 of 10; splits identically with Basque |
| K2 | Tagalog | 10 of 10 |
| K6 | Navajo, Hawaiian, Mongolian, Pohnpeian | 10 of 10 each; *kendi* settled the anchor in 1 of 49 triggers |
| K7 | Latin (ablative absolute) | 8 of 10; 23 and 56 already left unsplit by *-(y)lA* |
| K7 | Classical Chinese (Wényán) | 10 of 10 |
| K8 | Vigraha (compound resolution) | 10 of 10 |
| K8 | Ancient Greek (article & substantivization) | 0 of 10 |
| K8 | Polish (instrumental predication) | 3 of 3 |

No other roster lens was redundant in 7 or more of 10.

## Doubts

- **K3:** sentences that name their source were not counted as triggers. A
  looser reading would turn the 15 marker_only cases into both.
- **K3:** a suspended *-mIş* in coordination was read as non-evidential.
- **K4:** plural ethnonyms were treated as one bounded thing.
- **K2:**
  - Stance predicates (*görüşünde*) were counted as triggers.
  - 65 and 94 are misses of the blind pass, not marker errors.
  - Basque and Nez Percé split identically on all 10 sentences, so here they
    yield one measurement, not two.
- **Lens counts:** some 10 of 10 scores are partly true by construction,
  because Turkish has no counterpart at all (Tagalog, the K6 lenses).
- **K7:** whether *-(y)IncA* in 83 is temporal or causal.
