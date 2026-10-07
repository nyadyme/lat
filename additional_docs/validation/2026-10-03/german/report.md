# German routing profile: validation on 100 sentences

Files in this directory: `corpus.json`, `triggers.json` (blind, written before
the Routing section was read), `triggers_revised.json` (7 consistency fixes made
afterwards, flagged), `markers.json`, `counts.json` (blind triggers),
`counts_revised.json`, the scripts `build_corpus.py`, `triggers_src.py`,
`markers_src.py`, `compare.py`, and the fetched pages in `raw/`.

## 1. Corpus
- 60 Wikipedia sentences (de.wikipedia.org, MediaWiki plain-text extract, 12
  consecutive sentences each from the start of the article text):
  history *Dreißigjähriger Krieg* (1-12), natural science *Photosynthese* (13-24),
  politics/economics *Inflation* (25-36), biography *Lise Meitner* (37-48),
  culture/society *Oktoberfest* (49-60).
- 40 news/opinion sentences, 2 outlets: tagesschau.de news report on Steinmeier's
  Unity Day speech, 3 Oct 2026 (61-80); taz.de commentary *Das Beste für den
  Einheitstag ist das Wahlergebnis in Berlin*, 3 Oct 2026 (81-100). 7 URLs total.
- Deviations: the lead counts as article body (it holds full sentences). Bulleted
  lists after a colon were kept inside their sentence (4, 7). Sentence 31 contains
  a formula, kept as text. Skipped as non-article material: one embedded
  related-article teaser and the subheadings (tagesschau), one publisher promo box
  (taz). Four verbless taz units (84-86, 99) were kept, because they are
  consecutive full-stop units of the text. Wikipedia rate-limited the API, so I
  retried the same pages. No page was replaced.

## 2-4. Counts (blind triggers; revised in brackets where different)

| | both | trigger_only | marker_only | neither |
|---|---|---|---|---|
| K2 | 20 | 18 | 26 | 36 |
| K3 | 22 | 1 | 60 | 17 |
| K4 | 34 (35) | 8 | 2 (1) | 56 |
| K5 | 8 (11) | 0 | 11 (8) | 81 |
| K6 | 47 | 2 | 7 | 44 |
| K7 | 2 | 16 | 0 | 82 |
| K8 | 49 | 12 (15) | 3 | 36 (33) |

### Miss patterns
- **K2 trigger_only (18):** an agentless passive (15,16,17,20,22,25,39,47,48,50,79,85,88), *ist zu verdanken* (67), *gilt als* (60), a zu-infinitive with no controller (62), and a non-actor in the *durch*-agent slot of a nominalisation or participle (64,99).
  **marker_only (26):** 22 non-actor subjects of unaccusative or stative verbs (*entstehen*, *stattfinden*, *steigen*, *beginnen*, *haben*, *bezeichnen*, *entsprechen*, *lauten*: 1,7,12,21,24,26,27,30,34,35,36,40,51,55,56,57,58,89,91,93,95,100). The other 4 are collective subjects of action verbs (3,53,73,74). I annotated those as able to act; the profile counts them as triggers.
- **K3 marker_only (60):** the marker's first clause, "indicative assertion with no source named", matches almost every expository sentence: definitions, dates and documented facts (e.g. 13-21, 25-36, 39-45, 49-55). It even matches sentences that carry a lexical evidential (*gilt als* 5, 60; *überliefert ist* 59). The real trigger cases are inferred causes, intentions, collective attitudes, evaluations and predictions (1-4,6,8,22,38,46,57,73,81,82,87,88,90-92,95-98). The single trigger_only case (74) is an inference inside a quotation framed by Konjunktiv I.
- **K4 trigger_only (8):** a bare noun for a population (*Ost und West* 84,92,95), an indefinite article (*einer Volkswirtschaft* 26, *einen Umsatz* 54, *ein Viertel* 74), a demonstrative (*diesem Vorgang* 15), and proper names for powers (3).
- **K5 marker_only:** Präteritum where a duration adverbial keeps the process open (*ab 1912 arbeiteten* 41, *bis 1946* 44, *in den 30 Jahren folgten* 7, 6, 56), Präteritum used for reported present (72,73), and a Perfekt over a state that really did end (79).
- **K6 marker_only (7):** a non-possessive reflexive *sich* (2,87,90,91,92), a partitive genitive (4), and a genitive governed by a preposition (*angesichts* 82). **trigger_only (2):** a possessive *von*-phrase (99) and *haben* (89).
- **K7 trigger_only (16 of 18):** the cause is carried by:
  - prepositions: *durch* 64,97,99; *aufgrund* 50; *angesichts* 82; *anlässlich* 56; *für* 47; *mit* 9; *in Erinnerung an* 58; *auf … zurück* 57
  - adverbial connectives: *infolgedessen* 4, *folglich* 35, *deshalb* 80, *nämlich* / *mit der Folge* 30
  - a correlate: *daran, dass* 8
  - a comparison: *weniger mit … als mit* 95

  *nach* and *seit* occur only temporally (50: *seit 1810*; 73: *nach der Deutschen Einheit*).
- **K8 trigger_only (12):** a compound only in a predicative (1,13,29,37,90,97), an adjunct PP (2,9,39,48,50) or an attributive participle (23). **marker_only (3):** compounds that are not two nouns (*Sauerstoff* 20, *Zusammenarbeit* 40, *Selbstbehauptung* 80).

## 5. Verdicts
All German rows are `fires`, and none should flip to `inverted`.
- **K5:** grammar forced completion in 0 of 11 trigger sentences. Lexical Aktionsart (*ent-laden*, *aus-tragen*, *ver-binden*, *gelingen*, *ein-treten*, *ent-decken*, *er-halten*) carried it in 7 of 11, so "never marked" overstates the case: completion is never *grammatically* marked.
- **K3:** Konjunktiv I appears in 0 of 23 trigger sentences. Among the 11 sentences of reported speech in the news text, 6 use Konjunktiv I or II (67,68,74,76,78 and the clause in 75). Lexical evidentials appear in 4 Wikipedia sentences (5,58,59,60).
- **K6:** *dessen/deren* marks a non-subject possessor in 4 of 10 possessive sentences (6,12,18,45). *sein/ihr* leaves it open in 6 of 10.
- **K8:** the trigger occurs in 61 sentences (64 after revision). Spelling compounds as one word never named the relation, so I found no sign that the trigger is weak in German. Upgrading provenance from `derived` to `run` is supportable.

## 6. Lens check (German has no Substitutions rows)
- **K3 Latin (oratio obliqua):** it forces a choice German left open in 10 of 10 trigger sentences (1,2,3,4,6,8,22,38,46,57). It is redundant in 5 of 5 reported sentences that already use Konjunktiv I or II (67,68,74,76,78). It stays.
- **K3 Tuyuca:** forces a choice in 10 of 10. **Ewe:** neither redundant nor active; it applies in only 1 of these 10 sentences, because there is no report with a pronoun. Its one live case in the corpus is 78 (*er habe gedacht*).
- **K6 Mongolian:** forces a new choice in 6 of 10 (3,23,45,47,58,72). In 4 of 10, German *dessen/deren* already makes it. In all 6 open cases the context settles the reference trivially. It stays.
- **K5 Russian and Guaraní:** force a choice in 11 of 11. **Finnish:** applies only where there is an object (5 of 11). None is redundant.
- **K2 Tagalog (other lens):** its question, "which participant the event is told from", is already answered by German's choice of voice (active, passive or reflexive) in 10 of 10 K2 sentences (2,4,5,6,8,9,10,11,15,16). Only the answer space is wider in Tagalog (locative, instrument, beneficiary). Tentative, and probably not specific to German.
- **K8 Ancient Greek (article & substantivization) (other lens):** German has the same article-driven substantivization (*das Ausscheiden* 11, *des Aufbauens und Erneuerns* 73, *dem Bemühen* 75, *das Andenken* 78, *des Zusammenwachsens* 84, *das Erstarken* 96). In 10 of 10 K8 sentences (1,2,3,4,5,7,9,10,11,12), every element is already an article-marked noun. Tentative candidate for a German substitution (drop, or replace).
- **K4:** Yucatec forces a choice in 10 of 10 (German individuates by default). Polish (numeral threshold) applies in only 1 of 10 (7), and Hungarian in 3 of 10. Idle, but not redundant.

## 7. Proposed changes (German rows of the Routing table)
1. **K7 marker (clear; 16 of 18 trigger cases missed).**
   - Current: *weil*, *da*, *nach*, *seit* in a causal function.
   - Proposed: *weil*, *da*, *denn*; causal adverbs *deshalb*, *daher*, *folglich*, *infolgedessen*, *nämlich*; causal prepositions *durch*, *wegen*, *aufgrund*, *infolge*, *angesichts*, *anlässlich*, *dank*; *daran/darauf, dass* after verbs of failing or tracing back; and *nach* or *seit* only where they read as causal.
   - Evidence: 4, 8, 30, 35, 47, 50, 56, 64, 80, 82, 97, 99.
2. **K2 marker (clear; 18 trigger_only, 26 marker_only).**
   - Current: collective or non-actor as subject: *die Regierung beschloss*, *der Markt regelt*.
   - Proposed: collective or non-actor as subject of an action verb, or a passive or passive-equivalent with no agent: *wurde beschlossen*, *gilt als*, *ist zu tun*, *es wird erinnert*. Exclude unaccusative and stative verbs (*entsteht*, *findet statt*, *steigt*).
   - Evidence: the passive/equivalent group 15-17, 20, 22, 25, 39, 47, 48, 50, 60, 67, 79, 85, 88 (16 sentences), plus 22 unaccusative false hits.
3. **K3 marker (clear; 60 marker_only against 22 both).**
   - Current: indicative assertion with no source named; reported content in the indicative instead of Konjunktiv I.
   - Proposed: indicative assertion of inferred, evaluative or borrowed content (causes, intentions, collective attitudes, predictions) with no source, no Konjunktiv I and no lexical evidential (*gilt als*, *vermutlich*, *dürfte*, *überliefert*).
4. **K4 marker (clear; 8 trigger_only).**
   - Current: definite singular article for a bundle, mass or population.
   - Proposed: a singular noun phrase for a bundle, mass or population, whether definite, demonstrative, indefinite (*eine Volkswirtschaft*, *ein Viertel*) or bare (*Ost und West*).
   - Evidence: 15, 26, 54, 74, 84, 92, 95.
5. **K6 marker (tentative).**
   - Current: genitive, possessive pronoun, reflexive.
   - Proposed: genitive attribute (not partitive, not governed by a preposition), possessive pronoun, possessive *von*-phrase. Drop "reflexive": German has no reflexive possessive, and *sich* matched 5 non-possessive sentences (2, 87, 90, 91, 92). Add to the note: *dessen/deren* already marks a non-subject possessor (6, 12, 18, 45).
6. **K8 marker (tentative).**
   - Current: a compound in an argument slot.
   - Proposed: a noun–noun compound. Either drop "in an argument slot" (12 compounds sat in predicatives or adjuncts) or keep it as deliberate narrowing. "Noun–noun" removes 3 false hits (20, 40, 80).
   - Provenance `derived` can become `run` (trigger in 61 of 100).
7. **K5 note (tentative).**
   - Current: aspect is not grammatical, so completion is never forced and never marked.
   - Proposed: … never forced; lexical Aktionsart (*ent-*, *aus-*, *ver-*, *er-*) often marks it (7 of 11). The marker should not fire where a duration adverbial (*ab 1912*, *bis 1946*) keeps the process open (6, 7, 41, 44, 56).
8. **Possible new German Substitutions (tentative).**
   - K8 Ancient Greek (article & substantivization): German shares the device (10 of 10).
   - K2 Tagalog: German voice already selects the pivot (10 of 10), though this applies to every profile.

The verdicts (all `fires`) hold.

## Doubts
- **K2:** whether collectives "cannot act". I said they can. The marker says they cannot, which explains 4 of the 26 marker_only cases.
- **K3:** the trigger boundary between documented facts and inference. Wikipedia history sentences carry most of the risk (1-4, 6).
- **K4:** which abstract nouns count as a bundle. I excluded nouns in *-ismus* and *Demokratie*, and included wars, *Preisniveau* and *Inflation*. 29 is my own inconsistency.
- **K5:** strongly judgement-dependent. In the blind pass I missed 46, 47 and 55 by my own standard.
- **K6:** I counted every genitive attribute, including objective genitives. Narrower readings would cut *both*.
- **K8:** my reading of "argument slot" (subject, object or prepositional object, including NPs embedded in them) decides the 12 trigger_only cases.
- Seven trigger cells were revised after Step 3 (flagged in `triggers_revised.json`). Only K4, K5 and K8 change, and no conclusion depends on them.
