# French routing profile — validation on 100 sentences

Files: corpus.json, triggers.json (written before the Routing section was read),
markers.json, counts.json, compare.py, raw/ (fetched sources, build scripts).

## 1. Corpus
- Wikipedia (60; 12 consecutive each, from the first lead sentence):
  Guerre de Cent Ans (history, 1-12), Photosynthèse (natural science, 13-24),
  Inflation (politics/economics, 25-36), Marie Curie (biography, 37-48),
  Cuisine française (culture/society, 49-60).
- News/opinion (40): The Conversation France, "Manger à l'ère des algorithmes"
  (commentary, 61-80, 20 consecutive from the first body sentence);
  franceinfo, Tennessee prison director resigns (81-89, the whole 9-sentence body);
  franceinfo, IEA strategic oil reserves (90-100, the whole 11-sentence body).
- 8 distinct URLs, 2 news outlets.
- Deviations: (a) Guerre de Cent Ans: three bullet-list items in the lead
  (verbless fragments: "La « grande dépression médiévale » : une série de…",
  "Les conflits réguliers…", "Le conflit dynastique…") skipped as non-sentences,
  so ids 1-12 = lead sentences 1-3, 5, 8-15 of the extract. (b) RFI (first choice)
  returned 403; replaced by franceinfo. (c) The franceinfo bodies are short, so
  two articles from that outlet were taken to make 20. (d) Standfirsts, "Read
  more"/"Cinq questions…" link boxes were skipped as non-article text.
  (e) Sentence 84 is ungrammatical in the source ("…a survécu à deux injections
  létales a été laissée…") and kept verbatim.

## 2. Counts (sentence level)

| K | both | trigger_only | marker_only | neither |
|---|---|---|---|---|
| K2 | 8 | 59 | 6 | 27 |
| K3 | 0 | 10 | 16 | 74 |
| K4 | 26 | 6 | 3 | 65 |
| K5 | 7 | 3 | 3 | 87 |
| K6 | 37 | 1 | 33 | 29 |
| K7 | 2 | 12 | 0 | 86 |
| K8 | 39 | 5 | 12 | 44 |

Of the 8 K2 "both" cases only 3 (29, 73, 98) have the marker on the trigger span
itself; in 56, 65, 89, 90, 95 the marker (collective subject / *on*) and the
trigger (an agentless passive elsewhere in the sentence) only co-occur.

## 3. Miss patterns

**K2 (59 trigger_only).** The surface forms that carried the trigger:
- agentless passive, *être* + participle or bare participle (27): 2 *établie*,
  3 *sont catalysées*, 5 *est divisée*, 10 *est sacré*, 12 *cédés*, 14, 17, 18,
  24 *est fixé*, 30, 31 *est évalué*, 33 *est employé*, 37 *naturalisée*, 39, 41,
  44 *est conservée*, 55, 58, 59, 60 *est ajouté*, 74 *ce qui est proposé*, 84,
  85, 86 *est hospitalisée*, 87, 91 *les stocks libérés*;
- non-actor (abstraction, process, document, nominalisation) as subject or
  par-agent (27): 1, 4, 13 *le processus qui permet*, 16, 19, 20, 22, 23, 27,
  34, 35 *l'IPC intègre*, 36 *le concept induit*, 45, 49, 51, 57 *la diversité
  rend*, 64 *cette évolution suscite*, 67, 68 *nos travaux invitent*, 70, 75,
  76 *l'analyse révèle*, 79, 80, 83 *une enquête doit déterminer*, 94, 96
  *deux facteurs compliquent*;
- nominalisation with the actor left out, not in subject slot (4): 8 *la
  signature du traité*, 52, 77, 81 *l'exécution*;
- pronominal (middle) passive (1): 92 *s'y sont ajoutés* (also 56 *s'opère*, a "both").
*on* occurs in only 2 of 100 sentences (29, 56). marker_only 6, 11, 46, 66, 93,
99: collective *agents* (France, Russie, Anses, AIE, la presse) — real actors,
and 5 of the 6 are already K4 triggers.

**K3 (0 both).** All 10 triggers are inference or causal reconstruction stated
in the plain indicative with no source and no hedge (3, 4 *remontent à*, 7, 10,
12, 15 *apparue il y a 2,87 milliards d'années*, 16 *sont à l'origine de*, 17,
55, 56). None is a "report", so a marker that names reports misses them; the
conditional of conjecture (*serait apparue*) was available in all 10 and used in
none. All 16 marker_only are reports whose source *is* named in the sentence
(*selon*, *a annoncé*, *a précisé*, *avait affirmé que*) or an unattributed
quotation (30) — the trigger ("asserted as observation") is absent there. The
corpus contains no reportive conditional at all.

**K4.** trigger_only: pronoun resuming the bundle (54 *Celle-ci*, 55 *Elle*);
indefinite collective noun (23 *une série de réactions… appelées cycle de
Calvin*, 75 *un corpus de 313 publications*); definite generic mass (19 *toute
la matière organique*, 29 *la monnaie… elle a elle-même une valeur*).
marker_only 34 *la zone euro*, 45 *l'Empire russe*, 81 *l'administration
pénitentiaire* — arguably my strictness at the trigger pass, not marker error.

**K5.** trigger_only: past participle without auxiliary (15 *apparue… devenue*,
28 *une fois installé*), nominal event name (16 *la Grande Oxydation*).
marker_only: passé composé where the sentence itself keeps the process open
(90 *à ce jour*, 92 explicit span, 98).

**K6.** The *de*-genitive is in ~70 % of sentences; 33 marker_only are mostly
argument genitives of nominalisations (20 *l'absorption de l'énergie*, 52, 77,
95) and partitive/quantity or place/name *de* (24, 27, 34, 94, 96), not
possession. trigger_only 48 *ils ont eu trois filles* (possession by *avoir*).

**K7.** The four listed connectives cover 2 of 14. The causes were carried by
*grâce à* (12), *à la suite de* (10, 46), causal *pour* + noun (40, 41, 42, 43,
89 *pour ses travaux*, *condamnée pour le meurtre*), causal *par* (37 *par son
mariage*, 93 *par obligation gouvernementale*), *à l'origine de* (16), *avec* +
nominalisation (56). Also present but excluded as two causes: *en raison de* (55).

**K8.** trigger_only: N–N juxtaposition with no preposition (22 *dinucléotide
nicotinamide-adénine phosphate*, 38 *prix Nobel*, 40 *médaille Davy*, 74
*comptes avatars*; also in "both" sentences 34 *pays membres*, *zone euro*, 93
*Etats membres*, 68 *publications TikTok*) and a predicative N de N (47
*professeur de mathématiques*, not in an argument slot). marker_only 12: N de N
with an article-marked generic complement (26 *prix des biens*, 97 *prix des
carburants*, 77 *comptage des calories*) and institution names — these are
genuine N+N pairs; my trigger pass was too narrow, so these do not count
against the marker. Not counted on either side: relational adjectives
(*énergie lumineuse*, *pression fiscale*, *administration pénitentiaire*,
*contrôle alimentaire*), present in 20+ sentences.

## 4. Verdicts
All French rows say `fires`.
- K2, K4, K6, K7, K8: no French grammar forced the choice in any trigger
  sentence. `fires` holds.
- K3: conditionnel available in 10/10 trigger sentences, used in 0. `fires` holds.
- K5 (note: "the past already splits perfective from imparfait"): of the 10
  trigger sentences, 9 already encode completion grammatically — 7 passé
  composé (17, 54, 58, 62, 81, 91, 97), where the imparfait was available and
  not taken, and 2 resultative participles (15, 28). Only 16 (a nominal) leaves
  it open. No K5 trigger turned up in present-tense sentences, although the
  Wikipedia texts use the historical present heavily (6, 9, 12, 42, 43). In this
  corpus the row therefore behaves like `inverted` wherever it fires. 81 shows
  why that test is worth running: *a démissionné* forces a completion that 82
  contradicts (*démissionnera ce mois-ci*).

## 5. Lens checks
No Substitutions rows exist for French. Noted overlaps checked:
- **K5 / Russian** (10 trigger sentences): (a) forces a choice the sentence has
  not already made in 1/10 (16); redundant in 9/10. There is no replacement
  (b n/a). Hindi-Urdu (vector verbs), the replacement used for Polish, would
  force a new choice (whom the outcome falls to, how it came about) in 9/10
  (all except the verbless 16).
- **K5 / Finnish (partitive object)**: it is not redundant, but it has little
  to attach to, because only 1 of the 10 (91) has a direct object.
- **K4 / Yucatec Maya** (first 10 K4 triggers: 1, 2, 3, 4, 5, 6, 9, 11, 19, 23):
  French articles had already settled count against mass in 10/10, so (a) = 0/10.
  The same is true for German and English, which keep the lens, so I read it as
  a test of whether the forced choice was warranted, not as redundancy.
  Tentative only.
- Other roster lenses: none was redundant in 7 or more of 10. Mongolian's
  question is left open by *son* (as the note says), Tuyuca and Latin (oratio
  obliqua) are not forced, and Basque, Nez Percé and Tagalog are not forced.
  Polish (instrumental predication) has no copular predicate to attach to in
  6 of the first 10 K8 triggers. That means it is idle there, not redundant.

## 6. Proposed changes
Clear:
1. French K2 marker. Current: "the impersonal *on* or a collective as subject: *on a construit la
   bombe*, *l'État décide*". Proposed: "the agentless passive (*être* + participle
   with no *par*-phrase, or a bare participle: *la bombe a été construite*,
   *les territoires cédés*), the pronominal passive (*s'opère*), the impersonal *on*, or a
   non-actor (abstraction, nominalisation, document) as subject: *l'analyse révèle*,
   *cette évolution suscite*". Note: "*on* lets a sentence have a doer without
   naming one; in written French the agentless passive does this far more often."
   Evidence: 59 trigger_only (27 passive, 27 non-actor subject); *on* 2/100;
   collective subjects 6/6 marker_only.
2. French K3 marker. Current: "a report in the indicative where the *conditionnel journalistique* was
   available". Proposed: "an indicative assertion with no source named over borrowed
   or inferred content, where the conditional (journalistique or of conjecture:
   *il aurait signé*, *serait apparue*) was available". Note add: "a named source
   (*selon X*, *a annoncé*) already marks the report." Evidence: both 0,
   trigger_only 10 (all inference), marker_only 16 (all source-named).
3. French K7 marker. Current: "*parce que*, *car*, *puisque*, *à cause de*". Proposed: add "*grâce à*,
   *en raison de*, *à la suite de*, causal *pour* + noun (*pour ses travaux*),
   causal *par* (*par son mariage*), *être à l'origine de*". Note add: "*grâce à* adds a
   positive evaluation like Russian *благодаря*." Evidence: 12 trigger_only and 2
   both; *pour* 5, *à la suite de* 2, *par* 2.
4. French K5. Add a substitution row: "French | K5 | Russian | Hindi-Urdu (vector
   verbs) | in the past, the passé composé/simple vs imparfait split already
   forces the perfective choice". Alternatively keep Russian and add to the note:
   "in the past the row tests the forced choice as in an inverted row". Evidence:
   9/10 encoded, Russian redundant 9/10.
5. French K6 marker. Current: "*son / sa / ses* and the *de*-genitive". Proposed: "*son / sa / ses /
   leur*, *dont*, and the *de*-genitive with a possessed or relational head
   (kin, part, attribute: *la mère de X*, *la valeur de la monnaie*). This
   excludes the argument genitive of a nominalisation (*l'absorption de l'énergie*)." Evidence: 33
   marker_only out of 70 marker hits.
6. French K8 marker. Add "or N–N juxtaposition: *prix Nobel*, *zone euro*,
   *comptes avatars*". Evidence: 22, 38, 40, 74 (+34, 68, 93 in "both").
Tentative:
7. French K4 marker: add "or a pronoun resuming one (*elle*, *celle-ci*) and
   indefinite collective nouns (*une série de*, *un corpus de*)" (54, 55, 23, 75).
   The note's premise (partitive marks mass) held only for non-generic mass.
   Generic mass took the definite singular (19, 25, 29, 13).
8. French K8: consider the relational adjective (*énergie lumineuse*,
   *administration pénitentiaire*) as Russian and Polish do. It is frequent (20+),
   but the language-neutral trigger says "two nouns".

## 7. Doubts
- K2: counting agentless participles and nominalisations as "actor left out"
  inflates the count. Even if only finite passives and non-actor subjects are
  counted, trigger_only is still about 45.
- K3: I limited the trigger to inference and causal reconstruction and did not
  count documented facts or news (85). Read broadly ("everything in Wikipedia
  is borrowed"), the trigger and the marker would both fire almost everywhere.
- K4: I treated countries and institutions as populations only when they act
  as participants, not as places.
- K6 and K8 markers: deciding what counts as a *de*-genitive and what counts as
  N de N in surface terms involved judgement. In particular, "N de + generic
  article" was counted for K8.
- K5 81: I read *a démissionné* as completion presented over a pending process
  (82). One could read it as the act of handing in a resignation.
- K7: I applied "single cause" strictly, so *en raison de X et de Y* (55) is
  not counted, and I counted *après* in 81 as temporal.
