# Spanish profile — validation report

*Reconstructed verbatim from the annotating agent's final message; the agent
was not permitted to write report files. Sentence ids refer to `manifest.json`.*

Spanish profile on 100 sentences: every one of the seven verdicts holds, and the
K6 roster holds. Six of the seven markers either miss real triggers or fire
without one.

**Corpus.** 60 Wikipedia sentences, 12 consecutive from each of five articles:
Guerra de la Independencia (history), Fotosíntesis (natural science), Inflación
(economics), Cervantes (biography), Flamenco (culture). 40 news/opinion
sentences: BBC Mundo 14 (news), elDiario.es 13 (news), Público 13 (opinion
column). 8 URLs. Deviations: El País opinion showed a JS/ad-block wall, so
Público was taken instead; paywall banners and an image caption were skipped;
two splits that fell inside quotations were merged (ids 8 and 90); id 52 is a
fragment the source punctuates as a sentence and was kept.

| | both | trigger_only | marker_only | neither |
|---|---|---|---|---|
| K2 | 28 | 27 | 4 | 41 |
| K3 | 5 | 12 | 9 | 74 |
| K4 | 22 | 5 | 37 | 36 |
| K5 | 10 | 0 | 17 | 73 |
| K6 | 42 | 2 | 34 | 22 |
| K7 | 7 | 15 | 0 | 78 |
| K8 | 26 | 8 | 14 | 52 |

**Proposed changes — clear**
- K2: "impersonal, passive or anticausative *se*; the agentless *ser* passive or
  bare participle (*fue condenada*, *publicada*); a non-actor or nominalisation
  as subject (*la semana de la vivienda sustituyó*); the subjectless third
  plural (*nos echan*)", dropping plain pro-drop. 12 *ser*-passives and 12
  non-actor subjects among the misses; null subjects gave 4 false hits (10, 41,
  83, 89) and one true one (78).
- K3: "an inference, interpretation or borrowed claim in the plain indicative
  with no source named, where a conditional or a hedge was available". 12 misses
  were the writer's own inferences; 9 false hits named the source (*según*,
  *dijo que*, *reveló que*). The rumour conditional occurred 0 times in 40 news
  sentences.
- K4: "a definite singular collective or institution in an agent slot (*la
  población*, *la multitud*, *el Gobierno*), plus a bare proper name of a state
  or firm acting as one (*España*, *Gallup*)", with a note that the generic
  definite of mass and abstract nouns is obligatory. 33 false hits on *la
  inflación*, *la vida*, *el flamenco*; 3 missed bare names (1, 2, 73).
- K5: keep the marker, add "not over counted discrete events or the same-day news
  perfect" (68, 69, 70, 75).
- K6: add possessive determiners of every person and *cuyo* (10 and 78 missed:
  *nuestro*, *nuestras*; *mi* in 84). Exclude *de* after a deverbal or event noun
  (15 false hits), a place or naming apposition (10), a partitive (2), a head
  that names the relation (7).
- K7: add *gracias a*, *dado que* / *dada*, causal *por* + noun or infinitive,
  *por tanto* / *por ello*. The marker caught 7 of 22 triggers; misses: causal
  *por* 8, *gracias a* 2, *dado que* 2, *por tanto* / *por ello* 2.
- K8: add N+N juxtaposition (*pieza clave*, *Estado satélite*, *pigmento
  clorofila*: 3 misses) and *de* + article naming a type (*semana de la
  vivienda*: 5 misses); exclude a deverbal head with its theme (*expulsión de
  migrantes*: 7 false hits).

**Tentative**
- K5: exclude the indefinido of a completed life (*fue un novelista*, id 37).
- K8: Ancient Greek (article & substantivization) looks redundant: bare *de*
  against *de* + article already decides "type or bounded thing" in 7 of 10.

**Verdicts.** All seven `fires` hold. For K5 the past tense encodes the axis
about 40 % of the time: imperfecto or a periphrasis in 13 sentences against
indefinido or perfecto in 18.

**Lenses.** No Substitutions rows for Spanish.
- K5: Russian forces a new choice in 7 of 10, because the indefinido marks
  boundedness, not result. Finnish forces one in all 3 sentences that have an
  object and has nothing to attach to in the other 7. Guaraní 10 of 10. So the
  Polish Russian → Hindi-Urdu substitution should not be copied.
- K6: Mongolian 10 of 10, since *su* never shows whether the possessor is the
  subject. Keep.
- K3: Tuyuca 10 of 10; Latin oratio obliqua applies to 2 of 10; Ewe has nothing
  to attach to.

**Doubts.** K3 and K4 triggers were read narrowly; the profile's own example *el
capitalismo* suggests a broader reading of "bundle". Whether a head like
*pareja* or *presidente* already declares the kind of possession is a judgement.
Anticausative *se* was counted as the K2 marker, and the passive *por*-agent of a
causal verb as causal *por*. Counts are per sentence. The blind pass missed 9
triggers (8 for K5, 1 for K4) that the marker pass exposed; they are reported as
misses rather than re-annotated.
