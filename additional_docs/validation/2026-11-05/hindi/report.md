# Hindi profile: validation report

*The annotating agent was not permitted to write report files. This report was
reconstructed from its final message. Sentence ids refer to `manifest.json`.*

There was one annotator, one pass and no second rater.

All verdicts hold, the inverted K5 included. The weak points were:
- The K6, K5 and K3 markers fired far too often.
- K2, K4 and K7 missed real triggers.
- The derived removal of Russian from K5 was not supported. It has been
  withdrawn.

## Corpus (7 URLs)

The corpus has 100 sentences:
- **Wikipedia, 60 sentences.** 12 consecutive sentences from the start of each of five
  articles: मुग़ल साम्राज्य (history), प्रकाश-संश्लेषण (science), मुद्रास्फीति
  (economics), प्रेमचंद (biography) and दीपावली (culture).
- **News, 40 sentences:**
  - a Jansatta opinion column (ids 61–80);
  - a BBC Hindi news report (ids 81–100).

  Both are of 2026-11-05. Each was the most recent on its index page.

Deviations:
- The corpus was built before annotation began, by a separate script.
- A paragraph whose last sentence lacks a danda counts that tail as a
  sentence. Without this rule the first sentence of the history article was
  lost.
- Captions, teasers and BBC list items were removed by rule.
- Fragments and malformed sentences were kept so each run stays consecutive
  (2, 14, 21, 49).
- BBC Hindi ends sentences with "." and the other sources with "।". A
  quotation of two sentences spans two ids (86–87, 96–98).

## Counts

These are the markers **before** revision.

| | both | trigger_only | marker_only | neither | recall | precision |
|---|---|---|---|---|---|---|
| K2 | 44 | 13 | 9 | 34 | 77% | 83% |
| K3 | 54 | 0 | 36 | 10 | 100% | 60% |
| K4 | 26 | 10 | 14 | 50 | 72% | 65% |
| K5 | 20 | 1 | 32 | 47 | 95% | 38% |
| K6 | 45 | 0 | 35 | 20 | 100% | 56% |
| K7 | 8 | 3 | 2 | 87 | 73% | 80% |
| K8 | 35 | 0 | 43 | 22 | 100% | 45% |

## Miss patterns and changes applied

**Clear:**

1. **K6.** The marker is now restricted to genitives of belonging.
   - The 35 marker_only cases break down as 12 nominalisation arguments, 6
     composition or measure, 4 temporal, 3 titles and 10 abstract relations.
     None of them is possession.
   - The note already sent such genitives to K8; the marker text did not.
2. **K5.** The habitual now counts only in the past (फैलता था).
   - 27 of the 32 marker_only cases are generic present habituals, and none
     of them is a trigger.
3. **K3.** Added माना गया है, माना जा रहा है, देखा जा रहा है, X मानते हैं,
   अनुमान लगाया गया and के मुताबिक़.
   - The forms listed before occurred 5 times in 100 sentences, 2 of them as
     naming verbs (25, 55).
   - शायद, संभवतः, के अनुसार and बताया जाता है occurred 0 times; the
     presumptive occurred once.
   - Evidence: 5, 28, 59, 82, 83, 84, 89, 92, 95.
4. **K7.** Added के लिए ज़िम्मेदार (ठहराना) and की वजह / का कारण + copula (2,
   82, 86). Causal -कर occurred 0 times.
5. **K2.**
   - Added the dropped subject the preceding clause does not supply, and the
     subjectless infinitive (13, 14, 35, 42, 100).
   - The transitivity-pair clause now requires that an actor exists. Without
     that condition it fired on 15, 26, 29, 36 and 70.
6. **K4.**
   - "Bare" was dropped, because it missed 4, 9, 10 and 23.
   - A mass now counts only when it is delimited as one portion. 13 of the 14
     marker_only cases were unbounded masses.

**Tentative, kept as notes:**
- K4: a war, struggle or festival named as one thing (47, 49, 86, 88, 90).
  Three of these are the same noun in one article.
- K2: impersonal मिलना (48, 57).
- K5: the attributive perfective participle (46).

**Substitutions:**
- **K5: Russian → Luganda (graded tense) is withdrawn; Russian stays.**
  Russian still forced a choice in 6 of 10 sentences, because the Hindi simple
  perfective does not entail that the goal was reached, and the vector verb
  that would entail it is optional. Luganda forced one in 10 of 10, but
  trivially: the answer was always "remote", and 6 of the 10 were already
  dated.
- **K6: Mongolian stays dropped.** It forced a new choice in 0 of 10.
- **K8: Ancient Greek (article & substantivization) is newly dropped.** It
  forced a new choice in 0 of 10, because Hindi makes reification overt by
  derivation.

**Not applied:**
- **K8 marker.** The marker_only cases are mostly N का N. The agent read
  "joined" as compounding only. The marker keeps N का N, as French and Spanish
  keep N de N; counted that way, 24 of the 43 marker_only cases become both.
  Lexicalised compounds (राष्ट्रपति) were also missed in the blind pass.
- **K2 lenses.** Basque and Nez Percé were redundant in 9 of 10, partly by
  construction: the marker forms (passive, intransitive pair) answer their
  question. The same test would flag German, as the Japanese report notes,
  so both stay.
- **K4 lens.** Yucatec was redundant in 9 of 10 and stays, as for English,
  French and Arabic: it tests whether the settlement was warranted.

## K5 inverted check

Aspect was grammatically forced in 20 of 21 trigger sentences. The exception
is 2, where the process sits in the noun अंत. A vector verb added completion
in 4 sentences (7, 8, 31, 85), and the sentence itself bounded the process in
5. The verdict holds.

## Lens check

"Forces" means the lens forces a choice the Hindi sentence has not already
made. The per-sentence data is in `lens_check.json`.

| Comb. | Lens | Forces |
|---|---|---|
| K5 | Russian | 6 of 10 |
| K5 | Luganda (graded tense) | 10 of 10, trivial |
| K5 | Finnish (partitive object) | 3 of 5 (5 n/a) |
| K5 | Guaraní | 10 of 10 |
| K6 | Mongolian (reflexive possession) | 0 of 10 |
| K6 | Navajo (inalienable possession) | 10 of 10; के पास occurs 0 times |
| K6 | Hawaiian (a/o possession) | 10 of 10 |
| K6 | Pohnpeian (possessive classifiers) | 1 of 2 (8 n/a) |
| K2 | Basque (Euskara) | 1 of 10; ने settled only 1 |
| K2 | Nez Percé (Nimipuutímt) | 1 of 10 |
| K2 | Tagalog | 6 of 10 |
| K3 | Tuyuca | 10 of 10 |
| K3 | Ewe (logophoric pronouns) | not applicable in all 10 |
| K3 | Latin (oratio obliqua) | 6 of 10 |
| K4 | Yucatec Maya | 1 of 10 |
| K4 | Polish (numeral threshold) | 1 of 3 |
| K4 | Hungarian (associative plural) | 6 of 7; लोग occurs once, वग़ैरह 0 times |
| K7 | Latin (ablative absolute) | 10 of 10; no -कर |
| K7 | Classical Chinese (Wényán) | 10 of 10 |
| K8 | Vigraha (compound resolution) | 8 of 10 |
| K8 | Ancient Greek (article & substantivization) | 0 of 10 |
| K8 | Polish (instrumental predication) | 2 of 2 |

## Doubts

- **K8:** whether N का N is "joined", and the lexicalised compounds missed in
  the blind pass.
- **K3:** where description ends and inference begins in science and opinion
  sentences.
- **K2:** intransitive pair members where no actor exists.
- **K5:** the judgement that the Hindi perfective does not entail that the
  goal was reached, which carries the Russian count.
- **Pohnpeian:** whether it is not applicable to abstract possessions, or
  whether the general classifier always answers it.
