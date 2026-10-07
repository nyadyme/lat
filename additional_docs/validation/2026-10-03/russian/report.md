# Russian profile — validation report

*Reconstructed verbatim from the annotating agent's final message; the agent
was not permitted to write report files. Sentence ids refer to `manifest.json`.*

The verdicts and the substitutions hold; most markers miss the trigger often.
The Wikipedia API was rate-limited, so each article's HTML was fetched and its
paragraph text taken. `triggers.json` was written before the Routing section was
read.

**Corpus (7 URLs).** Wikipedia, 5 × 12 consecutive sentences: Куликовская
битва, Фотосинтез, Инфляция, Менделеев, Масленица. News and opinion, 2 × 20: a
BBC Russian report (Flydubai co-pilot attack) and a DW Russian commentary
(Preobrazhensky on the Kynev arrest). Four lead fragments without a verb were
left out; the quoted register entry plus its frame (sentence 42) count as one
sentence.

| K | both | trigger_only | marker_only | neither |
|---|---|---|---|---|
| K2 | 31 | 26 | 0 | 43 |
| K3 | 23 | 9 | 31 | 37 |
| K4 | 13 | 24 | 8 | 55 |
| K5 | 11 | 0 | 4 | 85 |
| K6 | 30 | 6 | 49 | 15 |
| K7 | 7 | 9 | 0 | 84 |
| K8 | 54 | 24 | 9 | 13 |

**Proposed changes — clear**
- K2: keep "3pl / -ся passive" and add agentless passive participles (*был
  убит*, *было обнаружено*: 8), actorless nominalisations (*задержание*,
  *обесценивание денег*: 9), a non-agent as subject of an action verb
  (*праздник маркирует*: 7), an impersonal modal with an infinitive (*следует*,
  *нельзя*: 3).
- K3: none of *якобы / мол / будто бы* occurs in the 100 sentences, so the
  marker does not discriminate. 19 marker_only sentences name their source (*по
  словам*, *сообщает*, quotes); 9 trigger_only sentences are the author's own
  inference. Proposed: "no source named (no reportive particle and no lexical
  attribution), content borrowed or inferred".
- K4: the triggers were specific collectives and institutions, not a generic
  bare singular. 24 misses (*войско*, *охрана*, *семья*, *власти*,
  *администрация*, *Кремль*); 8 false hits on plain mass nouns (*энергия*,
  *металл*).
- K6: genitives carry only 11 of 36 triggers; the rest are 20 possessive
  pronouns (7 of them *свой*), 4 *у*-phrases, 1 possessive adjective. 49
  marker_only sentences are non-possessive genitives. New marker: possessive
  pronouns, possessive adjectives, *у*-phrases and possessor genitives. New
  note: *свой* settles only whether the possessor is the subject, not what kind
  of possession it is.
- K7: add *вследствие*, *в результате*, *в связи с*, *так что* and causal *от*
  (9 misses).
- K8: add the bare adnominal genitive (*уровень цен*, *дело Кынева*); 23 of 24
  misses are genitive chains, so the genitive moves here from K6.

**Tentative**
- K5 marker: "perfective verb, participle or gerund" (7, 46, 49 are not finite).
- K8: clipped compounds (*генпрокурор*, *главред*, *Госдума*), about 8
  sentences, carrying the trigger alone only in 67.
- K4 note: agreement is free below five too (17: *было обнаружено два типа*).

**Verdicts and lenses**
- K5 `inverted` holds: all 11 trigger sentences carry the trigger through an
  obligatory perfective, the imperfective was available each time; the
  completion was warranted in 5 and questionable in 6.
- K5 Russian → Hindi-Urdu: removed lens 0 of 10, replacement 9 of 10.
- K6 Mongolian drop: would force a new choice in 1 of 10 (sentence 73, *его*
  inside an embedded noun phrase referring to the passive subject).
- K4 Polish (numeral threshold): 10 of 10. Keep.
- K8 Polish (instrumental predication): 10 of 10, since Russian predicate case
  follows the copula, not category against property. Keep; it has a predication
  to attach to in about 22 of 78 K8 sentences.
- Unprompted: K5 Finnish (partitive object) redundant or without an object in 9
  of 10 (settled by aspect in 6, no object in 3, new only in 49).

**Doubts.** Under a literal reading K6 and K8 triggers occur in almost every
sentence. Separating documented record from inference for K3 was a judgement
(biography sentences 37–48). Generic *инфляция* was not marked consistently for
K4. Some marker_only cases are blind-pass misses: the 9 relational adjectives in
K8, sentences 46 and 60 in K5; sentence 56 also has a *в связи с* missed for K7.
