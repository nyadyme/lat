# English routing profile — validation on 100 sentences

Files: corpus.json, triggers.json (blind, written before Routing was read), markers.json, counts.json, compare.py, split.py, raw/.

## Corpus
- 60 Wikipedia (en), 12 consecutive sentences each from the article start (lead included — the lead is where the article text begins; no sentence skipped):
  history = Hanseatic League (1–12), natural science = Photosynthesis (13–24), politics/economics = Inflation (25–36),
  biography = Ada Lovelace (37–48), culture/society = Jazz (49–60).
- 40 news/opinion, consecutive from the first body sentence: NPR, "The Supreme Court faces another term jam-packed with controversy" (Totenberg, 2026-10-03), text.npr.org/nx-s1-5986341 (61–80);
  The Guardian opinion, Arwa Mahdawi, "Pete Hegseth just took his personal crusade to a new level" (2026-10-03) (81–100).
- 7 URLs. Deviations: two NPR section headings ("Climate change", "Religion") skipped as non-sentences; "en" gloss = the text itself.

## Counts (per sentence)
| K | both | trigger_only | marker_only | neither |
|---|---|---|---|---|
| K2 | 20 | 27 | 24 | 29 |
| K3 | 11 | 0 | 70 | 19 |
| K4 | 12 | 40 | 16 | 32 |
| K5 | 3 | 13 | 1 | 83 |
| K6 | 27 | 11 | 29 | 33 |
| K7 | 2 | 9 | 0 | 89 |
| K8 | 41 | 10 | 0 | 49 |

## Miss patterns
- K2 trigger_only (27): agentless passive 12 (9,22,24,36,37,39,41,44,45,51,52,98); genre/style names as actors 6 (53,56–60: "Bebop ... shifting jazz", "Cool jazz ... introducing"); non-derived inanimate/abstract subjects 7 (10 power, 14 process, 16 cells, 23 derivative, 46 title, 47 exploits, 62 "the first Monday marked"); 6 actorless gerund; 78 "things have changed".
  marker_only (24): 21 institutional collectives acting (league, court, administration, Pentagon, military, government) + 3 nominalisations in copular subjects (13,21,29). If collectives are read as non-actors (doubt), ~16 of these move to "both"; the 27 misses remain.
- K3: marker fires on 81/100 sentences; it catches all 11 triggers but cannot separate them. The 11 triggers were: inner state/motive asserted (3,82,84,88,89,95), causal attribution (5,10,45), priority claim (38), paleo-reconstruction (19). 19 sentences carried explicit source/hedge (says, observes, it seems, apparently, allegedly, often considered, widely attributed, may/might).
- K4: of 52 triggers, 34 carried by a definite singular collective (the league, the court, the public, the military, the Pentagon, the government, the labor market), 14 by a bare singular mass/genre noun (jazz, bebop, photosynthesis), 4 by an indefinite singular collective (a network of, a collection of). The bare plural carried 0/52; its 12 "both" are co-occurrences elsewhere in the sentence. 16 marker_only bare plurals (traders, organisms, workers, women).
- K5: 13 trigger_only all simple past of change verbs, 8 with a period adjunct: expanded, evolved into, led to, became, evolved, pursued, originated, gave rise to, emerged, developed ×2, saw the emergence, found God. marker_only 51 ("has been recognized since": continuative, not completion).
- K6: trigger_only 11 = possessive determiner only (its/their/his/her: 2,7,9,41,42,48,50,64,91,99,100). marker_only 29 = non-possessive of-phrases: partitive/quantity (network of, number of, form of, billions of), event-argument (reduction of, emergence of, appointment of), titles (Earl of Lovelace), measures (rate of inflation).
- K7: trigger_only 9: adverb connectives thus/consequently (23,27,46), because (45), given (94), since (78), by + gerund (35), ground prepositions for/over in a VP or relative clause (72,95). Only 2/11 were causal prepositions inside an NP (32 out of, 33 due to).
- K8: trigger_only 10: stacks of 3+ nouns or coordinated compounds (21,52,54,62,73) and compounds in adjunct PPs (19,36,61,76,84). No marker_only — the marker is precise.

## Verdicts (all English rows "fires")
- All 7 hold. K5: progressive available but used 0/16 trigger sentences (used 9× elsewhere, all present/news) — completion never forced. K3: 0/11 triggers grammatically marked; English marks only lexically. K6: no reflexive possessive; optional "own" in 2 sentences (74,89). K4: English forces count/mass on the noun but not "one bounded thing" vs members.

## Lens check
- English has no Substitutions rows and no note naming a roster lens that overlaps English grammar.
- Unprompted (10 trigger sentences each, a = lens forces a choice the source has not made):
  K2 Basque 10/10, Nez Percé 10/10, Tagalog 0/10 strict — every English clause already chose an actor- or undergoer-pivot via active/passive; Tagalog's extra pivots (location, instrument, beneficiary) still contrast. Partial overlap, no removal.
  K3 Tuyuca, Ewe, Latin oratio obliqua: 11/11 a. K5 Russian, Finnish, Guaraní 10/10 a. K6 Navajo, Hawaiian, Mongolian, Pohnpeian 10/10 a (Mongolian: optional "own" 0/10 in sample).
  K4 Yucatec Maya: English had already fixed countability on the head noun in 10/10 (the league = count unit, jazz = mass) → meets the ≥7/10 redundancy threshold formally. The same holds for German, whose K4 row is run-confirmed, so this is more likely a limit of the redundancy test than an English-specific defect. Polish numeral, Hungarian associative, Mojeño 10/10 a.
  K8 Vigraha, Polish instrumental 10/10 a; Ancient Greek article: English has articles and free substantivisation, partial overlap, not counted.

## Proposed changes (Routing, English rows)
1. K4 marker (clear; 40 trigger_only, carrier hit 0/52): current "the bare plural for a population: *groups*, *races*, *workers*" + note "the trigger inverts against German: the absence of a determiner marks it, not a definite article" → proposed "definite singular collective or bare singular mass noun for a bundle or population: *the court*, *the public*, *the military*, *jazz*; bare plural as secondary"; drop the inversion note.
2. K2 marker (clear; 12 passives, 13 non-derived/genre subjects): current "a nominalisation or collective in subject position: *the analysis shows*, *inequality drives*" → "an agentless passive, or a non-human noun (nominalisation, process, genre, thing) as subject of an action verb: *was chosen*, *the analysis shows*, *jazz drew on*". Whether collectives stay depends on whether institutions count as able to act (doubt).
3. K7 marker (clear; 9/11 missed): current "a causal preposition inside a noun phrase: *because of*, *due to*, *as a result of*" → "*because*, *since*, *as*, *given*, *thus*, *consequently*, and causal prepositions (*because of*, *due to*, *out of*, *for*, *over*)"; keep the NP point as the note.
4. K5 marker (clear; 13 trigger_only): current "the present perfect over a running state: *has always worked*" → "simple past or present perfect of a change verb where the progressive was available, often with a period adjunct: *emerged in the 1940s*, *has always worked*" (cf. French row).
5. K6 marker (clear; 11 trigger_only, 29 marker_only): current "*of*-phrase and possessive *'s*" → "possessive *'s*, possessive determiners *his/her/its/their*, and an *of*-phrase naming a part, kin or property of its noun"; the note already discusses *his*, so the marker omitting it is inconsistent.
6. K3 marker (clear; 70 marker_only): current "indicative assertion with no source named" → "unsourced indicative over content nobody observed: a motive or inner state (*aiming to*, *is worried*), a causal attribution (*led to*, *enabled*), a priority claim (*the first to*)".
7. K8 marker (tentative; 5+5): "a two-word noun compound in an argument slot" → "a noun compound of two or more nouns"; whether adjunct-slot compounds count is a design question.

## Doubts
Collectives as actors (K2); whether a bare plural kind ("workers") counts as one bounded thing (K4); which of-phrases are possessive (K6 trigger frozen blind; e.g. "price of goods" was not marked though arguably a property relation); "inference asserted as observation" is interpretive (K3, 11 cases, mostly opinion piece); "since" in 78 read as causal; argument vs adjunct slot (K8); single annotator, no second pass.
