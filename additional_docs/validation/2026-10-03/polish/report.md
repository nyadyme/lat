# Polish profile — validation report

*Reconstructed verbatim from the annotating agent's final message; the agent
was not permitted to write report files. Sentence ids refer to `manifest.json`.*

The verdicts hold, including K5 `inverted`, and so do the K4, K5 and K6
substitutions. Most markers are too narrow, and the K6 marker pointed at the
wrong form.

**Corpus (7 URLs).** 60 Wikipedia sentences, 12 per article from the first
sentence: Bitwa pod Grunwaldem (history), Fotosynteza (science), Inflacja
(economics), Maria Skłodowska-Curie (biography), Kuchnia polska (culture). 40
news/opinion sentences: a Krytyka Polityczna interview (61–83) and a Polsat
News report (84–100). The split is 23/17 because the Polsat article has only
17 sentences; the Krytyka Polityczna standfirst and fact box were skipped; the
Wikipedia API was rate-limited and retried, no page was swapped.

| K | both | trigger_only | marker_only | neither |
|---|---|---|---|---|
| K2 | 7 | 34 | 6 | 53 |
| K3 | 3 | 1 | 11 | 85 |
| K4 | 11 | 17 | 11 | 61 |
| K5 | 1 | 4 | 5 | 90 |
| K6 | 20 | 5 | 48 | 27 |
| K7 | 4 | 10 | 1 | 85 |
| K8 | 41 | 27 | 17 | 15 |

**Proposed changes — clear**
1. K2: add impersonal *się* (10 sentences, e.g. 19, 29, 79), agentless *być /
   zostać* + participle (11, e.g. 2, 42) and inanimate subjects (9, e.g. 15,
   55). The *-no / -to* form appeared only twice (3, 41).
2. K3: require borrowed content with no source named; a verb of saying with a
   named speaker already names the source, which caused all 11 false hits (59,
   83–96).
3. K4: add singular collectives and institutions standing for their members
   (*zakon*, *wojsko*, *rząd*, *NBP*), 17 misses; exclude plain mass readings
   (*tlen*, *herbata*), 11 false hits.
4. K6: possessive pronouns, *swój*, *mieć* and the genitive of a possessor.
   Plain genitive chains (*poziom cen*) express no possession: 48 false hits, 5
   misses (12, 48, 58, 98, 100). *swój* settles only whether the possessor is
   the subject, not the kind of possession, so it should stay a marker.
5. K7: add *więc*, *wskutek*, *dzięki*, *w efekcie*, *zwłaszcza że*, *za* +
   accusative of reason, *z inicjatywy* (10 misses).
6. K8: add a noun with a bare genitive noun (*poziom cen*, *szef MSZ*), 27
   misses.
7. Substitutions, K8 reason: "own entry; the grammar forces the category/property
   choice only at a copula". The second half of the old reason failed in 6 of
   10; the removal stands.
8. New Substitutions row, Polish K5, Finnish (partitive object): redundant in 7
   of 10 (40, 55, 66, 85, 86, 89, 94); the other 3 are intransitive. Polish
   perfective aspect, the partitive genitive and the genitive of negation (10,
   46, 98) already say how far the event reached the object.

**Tentative**
- K3: add inferences about intentions stated without a hedge (97–99, 62).
- K5: add perfective verbal nouns (*rozwinięcie*, *oblężenie*), 2 cases.
- Hausa as a replacement in the new Finnish row.

**Verdicts and substitutions**
- K5 `inverted`: aspect forced in 10 of 10 relevant sentences; the forced
  completion was warranted in 6, 58, 59, overstated in 55, contested in 85, 86,
  89.
- K2, K3, K4, K7, K8 `fires`: confirmed. No *podobno*, *rzekomo* or *jakoby* in
  the corpus.
- K6: the reflexive choice was encoded 14 of 14 times; the kind of possession 0
  of 25.
- K4 Hausa replacement: removed lens 0 of 10, Hausa 8 of 10.
- K5 Hindi-Urdu replacement: Russian 0 of 10, Hindi-Urdu 9 of 10.
- K6 Mongolian drop: 0 of 10.

**Doubts.** Institutions were filed under K4, not K2. K3 "borrowed" was read as
reported speech. The blind K8 trigger counted only noun + noun, which inflates
marker_only. *ponieważ* in sentence 3 was not counted for K7 (three causes).
What counts as a running process for K5 is interpretive. Genitives of persons
were taken as possessive (K6), part-whole genitives as K8. During the run one
fetch briefly wrote a file into the repository; it was moved out at once.
