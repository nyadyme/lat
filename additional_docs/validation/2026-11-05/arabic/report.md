# Arabic (Modern Standard Arabic) profile: validation report

*The annotating agent was not permitted to write report files. This report was
reconstructed from its final message. Sentence ids refer to `manifest.json`.*

There was one annotator, one pass and no second rater.

All seven `fires` verdicts hold. The weak points were the marker texts:
- K7 and K2 missed many real triggers.
- K3 and K5 fired on far too much.

## Corpus (7 URLs)

The corpus has 100 sentences:
- **Wikipedia, 60 sentences.** 12 consecutive sentences from the start of each of five
  articles: الدولة العباسية (history), تركيب ضوئي (science), تضخم اقتصادي
  (economics), ابن خلدون (biography) and قهوة عربية (culture).
- **News, 40 sentences:**
  - an Al Jazeera opinion piece (ids 61–80);
  - a BBC Arabic news report (ids 81–100).

  Both are of 2026-11-05. Each was the first on its index page.

Deviations:
- The corpus was built before annotation began, by a separate script.
- Pull quotes, teasers and captions in the news pages were removed by rule:
  any paragraph without closing punctuation was dropped.
- The economics run includes 8 list-item fragments (26–29, 33–36). They were
  kept so the run stays consecutive, as for Japanese.
- A line that ends in ":" before a list counts as a sentence.

The Step-6 samples are the trigger ids of each combination, evenly spaced, not
picked by hand.

## Counts

These are the markers **before** revision.

| | both | trigger_only | marker_only | neither | recall | precision |
|---|---|---|---|---|---|---|
| K2 | 62 | 18 | 1 | 19 | 78% | 98% |
| K3 | 42 | 0 | 27 | 31 | 100% | 61% |
| K4 | 39 | 32 | 4 | 25 | 55% | 91% |
| K5 | 31 | 0 | 35 | 34 | 100% | 47% |
| K6 | 63 | 0 | 5 | 32 | 100% | 93% |
| K7 | 7 | 18 | 1 | 74 | 28% | 88% |
| K8 | 85 | 2 | 7 | 6 | 98% | 92% |

## Miss patterns

- **K2:** 18 trigger_only:
  - 8 are actorless verbal nouns without تمّ (29, 36, 44, 59, 69, 70, 98), plus 15 with يجري + verbal noun;
  - 2 are passive participles (50, 54);
  - 2 are form VI anticausatives (61, 71);
  - 3 are human collective plurals as actor (5, 57, 88);
  - 2 are verbs of occurring (78, 84), where the blind pass was probably too generous.
- **K3:**
  - 0 trigger_only.
  - 27 marker_only: 18 are encyclopaedic science, economics or description
    sentences; most of the rest are unattributed narration of public events.
  - Two of the marker_only cases (74, 100) are real triggers that the blind
    pass missed.
- **K4:** 32 trigger_only:
  - 21 are mass or abstract nouns, which the note excludes on purpose. The
    exclusion also catches masses bounded by a demonstrative or quantifier
    (14, 18, 21).
  - Other gaps: bare state names (72, 73, 78; 79 and 99 fired only through
    another noun), plurals (5, 37, 96) and indefinite collectives (32, 64).
- **K5:**
  - 0 trigger_only.
  - 35 marker_only; 32 of them come from the clause "a verbal noun over a
    process". It fires on every generic or definitional verbal noun.
  - In all 31 trigger sentences the writer chose the perfective, although
    كان + imperfect was available.
- **K6:**
  - 5 marker_only: relational لـ (26, 50, 58) and object suffixes on a verbal
    noun (83, 92).
- **K7:** 18 trigger_only. The list left out:
  - بفعل (22), من خلال (13, 23), بواسطة (51), بموجب (79), على خلفية (100);
  - instrumental بـ + verbal noun (11, 37, 45), بذلك (3, 32);
  - مما / ما (7, 74), الناجم عن / النابع من (63, 70);
  - causal لـ before a noun that is not a verbal noun (65, 95).
- **K8:**
  - The marker holds.
  - The 7 marker_only cases are nisba adjectives, probably real triggers that
    the blind pass read too literally ("two nouns").

## Changes applied to `lat_routing.md`

**Clear:**
- K2: add يجري + verbal noun and the actorless verbal noun.
- K3: add the note that plain exposition of documented fact is not the
  trigger.
- K4:
  - add bare names of states and cities standing for their government;
  - add the note that a mass bounded by a demonstrative or quantifier carries
    the trigger (3 sentences);
  - add the note that feminine singular agreement of non-human plurals is
    obligatory and does not carry it.
- K5: restrict the verbal noun to a past process in narrative.
- K6: restrict لـ to possessive لـ.
- K7: add بفعل, على خلفية, بذلك and the means prepositions من خلال, بواسطة,
  بموجب and instrumental بـ.
- K5 substitution: drop Finnish (partitive object), as for Russian and Polish
  (0 of 8 applicable).

**Tentative, kept as notes:**
- K2: form VI and passive participles.
- K6: object suffix on a verbal noun.
- K7: مما, ناجم عن, نابع من and causal لـ before any noun.

**Not applied:**
- The agent's tentative proposal to remove Yucatec Maya from K4 (1 of 10).
  The English and French checks found the article settling the Yucatec
  question in 10 of 10 and kept the lens, because it tests whether that
  settlement was warranted. The Arabic row follows that precedent and records
  the count in its note.

## Lens check

"Forces" means the lens forces a choice the Arabic sentence has not already
made. The per-sentence data is in `lens_check.json`.

| Comb. | Lens | Status | Forces |
|---|---|---|---|
| K5 | Russian | removed | 2 of 10: the removal holds; the exceptions are the durative states دام and عاش |
| K5 | Hindi-Urdu (vector verbs) | replacement | 7 of 10: holds |
| K5 | Finnish (partitive object) | roster, now dropped | 0 of 8 (2 n/a) |
| K8 | Ancient Greek (article & substantivization) | dropped | 0 of 10: the drop holds |
| K4 | Polish (numeral threshold) | roster | 6 of 8 (2 n/a); no numerals occurred; agreement settled it once (9) |
| K4 | Yucatec Maya | roster | 1 of 10 |
| K4 | Hungarian (associative plural) | roster | 4 of 7 (3 n/a) |
| K6 | Mongolian (reflexive possession) | roster | 5 of 10; نفسه occurs 0 times |
| K6 | Navajo (inalienable possession) | roster | 10 of 10 |
| K7 | Latin (ablative absolute) | roster | 9 of 10; the ḥāl clause exists (41), but only بعدما (67) left the relation open |
| K7 | Classical Chinese (Wényán) | roster | 10 of 10 |
| K2 | Basque / Tagalog | roster | 7 / 6 of 10 |
| K3 | Tuyuca / Latin (oratio obliqua) | roster | 9 / 9 of 10 |
| K3 | Ewe (logophoric pronouns) | roster | not applicable in all 10: no report frame |

## Doubts

- **K3:** the trigger is a judgement call. Unhedged history was counted,
  science was not. 74 and 100 were missed in the blind pass.
- **K2:** states and collectives were counted as non-actors, and the blind
  pass was generous on verbs of occurring.
- **K4:** part of the 32 trigger_only cases may be over-annotation of masses,
  given the obligatory article.
- **K6 and K2:** suffixes on verbal nouns were coded inconsistently.
- **K7:** the lens counts are near-trivial. A trigger sentence names its
  relation by definition, so any ablating lens "forces" a choice.
- **Finnish and Yucatec:** both rest on one sample of 10 each.
