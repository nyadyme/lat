<!-- SPDX-License-Identifier: LicenseRef-Proprietary -->
<!-- Copyright 2026 Dana Schlifka -->
<!-- Dual-licensed work: this proprietary edition is the one that applies in   -->
<!-- this repository. The `lat` edition of the same catalogue is offered under -->
<!-- Apache-2.0 and is a separate distribution. See the LICENSE file.          -->

# lat – Combination routing (canonical source)

This file is the **single source of truth** for the pass-2 combinations and for
how they are routed per source language. `src/routing_seed.sql` is **generated
from it** by `tools/gen_routing.py`, and the MCP tool `get_routing` serves it.

The combinations themselves are language-neutral: roles, polarity and the
interference rules are about lenses, and the trigger of each combination is
stated as a structural fact, not as a word. What depends on the source language
is everything a profile below records:

- **the marker** — the surface form that carries the trigger in that language;
- **the verdict** — `fires` when the language leaves the axis implicit and the
  combination applies as written, `inverted` when the language already forces
  the choice, so the combination tests whether the forced choice was warranted
  rather than whether one was made;
- **the substitutions** — a lens that is the source language itself, or that
  forces a choice the source grammar already forces on the same axis, is no
  contrast and leaves the roster. It is replaced by a lens that asks a
  different question on the same axis, or dropped when the catalogue holds
  none. Kinship is judged per axis, never per family.

`Provenance` says how a row was arrived at: `run` — read off a documented run of
the whole catalogue over texts in that language; `derived` — worked out from the
cited grammar without a run yet; `checked` — the marker was tested against 100
consecutive real sentences in that language (60 from Wikipedia across five
domains, 40 from news and opinion pages; 2026-10-03, Chinese and Arabic 2026-11-05) and revised where it
missed real triggers or fired without one. A `checked` row records the latest
evidence; its verdict may rest on an earlier run. The check had one annotator
per language and no second rater, so its counts show direction, not a
measurement. A note marked *tentative* rests on fewer than three sentences.
The corpus manifests, annotations, counts and reports of the check are in
[`validation/2026-10-03/`](validation/2026-10-03/README.md) and, for Chinese and Arabic,
[`validation/2026-11-05/`](validation/2026-11-05/README.md).

## Combinations

| Id | Name | Axis | Trigger | Provenance |
|---|---|---|---|---|
| K1 | base cycle | — | any sentence | run: ten German sentences |
| K2 | agent pincer | Agency & control | a participant that cannot act stands where the actor goes, or the actor is left out | run: ten German sentences |
| K3 | source pincer | Evidence & certainty | inference, report or memory is asserted as observation | run: ten German sentences |
| K4 | boundary pincer | Object boundaries | a population, mass or bundle is named as one bounded thing | run: ten German sentences |
| K5 | aspect pincer | Time & aspect | a running process is presented as a completed point | run: ten German sentences |
| K6 | possession pincer | Possession & belonging | a possessive relation stands without its kind declared | run: ten German sentences |
| K7 | simultaneity pincer | Causality | a single cause is asserted by a connective or preposition | run: ten German sentences |
| K8 | compound pincer | Object boundaries | two nouns are joined without the relation between them named | run: one English text |

## Rosters

`Role` is one of `opener`, `reduction`, `ablation`, `counter-check`,
`reconstruction`; `Polarity` one of `constructive`, `destructive`, `both`.

| Combination | Position | Lens | Kind | Role | Polarity |
|---|---|---|---|---|---|
| K1 | 1 | Node collapse | form | reduction | constructive |
| K1 | 2 | Word ban | form | ablation | both |
| K1 | 3 | Back-translation | form | counter-check | destructive |
| K1 | 4 | Form-switch | form | reconstruction | constructive |
| K2 | 1 | Basque (Euskara) | language | ablation | both |
| K2 | 2 | Nez Percé (Nimipuutímt) | language | ablation | destructive |
| K2 | 3 | Tagalog | language | reconstruction | constructive |
| K3 | 1 | Tuyuca | language | ablation | destructive |
| K3 | 2 | Ewe (logophoric pronouns) | language | ablation | destructive |
| K3 | 3 | Latin (oratio obliqua) | language | reconstruction | constructive |
| K4 | 1 | Yucatec Maya | language | ablation | destructive |
| K4 | 2 | Polish (numeral threshold) | language | ablation | destructive |
| K4 | 3 | Hungarian (associative plural) | language | ablation | destructive |
| K4 | 4 | Mojeño Trinitario (possessive classes) | language | ablation | destructive |
| K4 | 5 | Toki Pona | language | reduction | both |
| K4 | 6 | Swahili (noun classes) | language | reconstruction | constructive |
| K5 | 1 | Russian | language | ablation | destructive |
| K5 | 2 | Finnish (partitive object) | language | ablation | destructive |
| K5 | 3 | Guaraní | language | reconstruction | constructive |
| K6 | 1 | Navajo (inalienable possession) | language | ablation | destructive |
| K6 | 2 | Hawaiian (a/o possession) | language | ablation | destructive |
| K6 | 3 | Mongolian (reflexive possession) | language | ablation | destructive |
| K6 | 4 | Pohnpeian (possessive classifiers) | language | reconstruction | constructive |
| K7 | 1 | Fugue | form | ablation | both |
| K7 | 2 | Latin (ablative absolute) | language | ablation | destructive |
| K7 | 3 | Classical Chinese (Wényán) | language | ablation | both |
| K7 | 4 | Dialectical Midrash | form | reconstruction | constructive |
| K8 | 1 | Vigraha (compound resolution) | form | ablation | destructive |
| K8 | 2 | Ancient Greek (article & substantivization) | language | ablation | destructive |
| K8 | 3 | Polish (instrumental predication) | language | ablation | both |
| K8 | 4 | Lojban | language | reconstruction | constructive |

## Profiles

`Own entries` are every catalogue entry that describes the source language
itself, by exact name. They go into `exclude_names` for every search over a text
in that language, and none of them may ride in a combination for it.

| Source language | Own entries | Note |
|---|---|---|
| German | German | the language the routing was first derived on |
| English | English | K8 was derived on an English text |
| French | | the catalogue holds no French entry |
| Spanish | Spanish (differential object marking) | |
| Russian | Russian | |
| Polish | Polish (instrumental predication), Polish (numeral threshold), Polish (masculine-personal plural) | |
| Japanese | Japanese, Japanese (listing particles) | |
| Chinese | | Standard Mandarin; the catalogue holds no entry for it. Classical Chinese (Wényán) is a different language stage, not an own entry, and stays in K7: it forced a choice in 10 of 10 trigger sentences checked, since every one stated the causal relation it asks about |
| Arabic | Arabic (root & pattern) | Modern Standard Arabic, written |

## Routing

| Source language | Combination | Verdict | Marker | Note | Citation | Provenance |
|---|---|---|---|---|---|---|
| German | K1 | fires | any sentence | | Hawkins, A Comparative Typology of English and German (1986) | run |
| German | K2 | fires | an agentless passive or its equivalents: *wurde gebaut*, *gilt als*, *ist zu tun*, *es wird*; or a non-actor as subject of an action verb: *die Analyse zeigt*, *der Markt regelt* | verbs of occurring (*entsteht*, *findet statt*, *steigt*) are not action verbs and do not carry the trigger | Hawkins, A Comparative Typology of English and German (1986) | checked |
| German | K3 | fires | inferred, evaluative or borrowed content in the indicative with no source named, no Konjunktiv I and no lexical hedge such as *gilt als*, *vermutlich*, *dürfte*, *überliefert* | plain exposition of documented fact is not the trigger; a named source or Konjunktiv I already marks a report | Duden, Die Grammatik (9th ed., 2016) | checked |
| German | K4 | fires | a singular noun phrase standing for a bundle, mass or population, definite or not: *der Arbeiter*, *die Wirtschaft*, *Ost und West*, *eine Gesellschaft* |  | Duden, Die Grammatik (9th ed., 2016) | checked |
| German | K5 | fires | Perfekt or Präteritum over a running process: *hat immer funktioniert* | aspect is not grammatical, so completion is never forced; tentative: prefixed verbs (*ent-*, *aus-*, *ver-*, *er-*) often mark it lexically, and a duration adjunct (*ab 1912*) blocks the marker | Hawkins, A Comparative Typology of English and German (1986) | checked |
| German | K6 | fires | a possessive pronoun, a possessor genitive or a possessive *von*-phrase: *seines Lebens*, *dessen Folgen*, *das Haus von Anna* | *sein* does not mark whether the possessor is the subject, while *dessen / deren* marks a possessor that is not; a genitive that is the argument of a nominalisation is not possession and belongs to K8 | Duden, Die Grammatik (9th ed., 2016) | checked |
| German | K7 | fires | *weil*, *da*, *denn*; *deshalb*, *daher*, *folglich*, *infolgedessen*, *nämlich*; causal *durch*, *wegen*, *aufgrund*, *infolge*, *angesichts*, *dank*; *daran, dass* | *nach* and *seit* were temporal in every case checked; count them only where the reading is causal | Hawkins, A Comparative Typology of English and German (1986) | checked |
| German | K8 | fires | a noun–noun compound in any slot: *Planwirtschaft*, *Politikexperte* | German writes the compound as one word, which keeps it legible as a compound; restricting the marker to argument slots missed 12 of 61 cases | Duden, Die Grammatik (9th ed., 2016) | checked |
| English | K1 | fires | any sentence | | Huddleston & Pullum, The Cambridge Grammar of the English Language (2002) | derived |
| English | K2 | fires | an agentless passive, or a non-human noun as subject of an action verb: *was chosen*, *the analysis shows*, *jazz drew on* | English nominalises freely, so more abstractions reach the subject slot without any morphology | Huddleston & Pullum, The Cambridge Grammar of the English Language (2002) | checked |
| English | K3 | fires | unsourced indicative over content nobody observed: a motive or inner state, a causal attribution, an evaluation or priority claim | English marks the source only lexically (*says*, *seems*, *allegedly*); a named source already marks the report | Huddleston & Pullum, The Cambridge Grammar of the English Language (2002) | checked |
| English | K4 | fires | a definite singular collective or a bare singular mass or genre noun: *the court*, *the public*, *jazz* | the bare plural carried the trigger in none of 52 cases checked; the earlier claim that the trigger inverts against German does not hold. The article settled count against mass in 10 of 10 sentences checked, and Yucatec Maya stays: it tests whether that settlement was warranted, as in German | Huddleston & Pullum, The Cambridge Grammar of the English Language (2002) | checked |
| English | K5 | fires | simple past or present perfect of a change verb over a span, where the progressive was available: *emerged in the 1940s*, *has always worked* | the progressive was available and unused in all 16 trigger sentences checked | Huddleston & Pullum, The Cambridge Grammar of the English Language (2002) | checked |
| English | K6 | fires | possessive *'s*, a possessive determiner (*its*, *their*, *his*), or an *of*-phrase naming a part, kin or property of its noun: *the mistake of his life* | *his* leaves open whether the possessor is the subject; partitive, title and event *of*-phrases are not possession | Huddleston & Pullum, The Cambridge Grammar of the English Language (2002) | checked |
| English | K7 | fires | *because*, *since*, *as*, *given*, *thus*, *consequently*, and causal prepositions: *because of*, *due to*, *out of*, *for*, *over* | a cause often hides in a noun phrase instead of opening a clause, and the word ban has to target the preposition then | Huddleston & Pullum, The Cambridge Grammar of the English Language (2002) | checked |
| English | K8 | fires | a noun compound of two or more nouns in any slot: *group differences*, *stereotype threat* | the pair reads as modifier and noun, and the case relation between the members vanishes without trace; the earlier marker, two nouns in an argument slot, missed 5 compounds in adjuncts and 5 stacks of three or more nouns or coordinated compounds | Huddleston & Pullum, The Cambridge Grammar of the English Language (2002) | checked |
| French | K1 | fires | any sentence | | Riegel, Pellat & Rioul, Grammaire méthodique du français (1994) | derived |
| French | K2 | fires | an agentless passive (*être* + participle without *par*, or a bare participle), the pronominal passive (*s'opère*), *on*, or a non-actor as subject: *l'analyse révèle*, *cette évolution suscite* | *on* occurred in only 2 of 100 sentences checked; a collective that really acts (*la France*, *la presse*) is a K4 case, not a K2 one | Riegel, Pellat & Rioul, Grammaire méthodique du français (1994) | checked |
| French | K3 | fires | inferred or borrowed content in the indicative with no source named, where the conditional (*journalistique* or of conjecture: *serait apparue*) was available | a named source (*selon*, *a annoncé*) already marks the report; the conditional did not occur in the 100 sentences checked | Dendale, Le conditionnel de l'information incertaine (1993) | checked |
| French | K4 | fires | definite singular for a bundle or population: *l'ouvrier*, *le capitalisme* | generic mass nouns took the definite singular rather than the partitive in the sentences checked; tentative: resuming pronouns (*celle-ci*) and indefinite collectives (*une série de*) carried it too. The article settled count against mass in 10 of 10 sentences checked, and Yucatec Maya stays: it tests whether that settlement was warranted, as in German | Riegel, Pellat & Rioul, Grammaire méthodique du français (1994) | checked |
| French | K5 | fires | passé composé over a running state: *a toujours bien fonctionné* | the past already splits perfective from imparfait, and the passé composé encoded completion in 9 of 10 trigger sentences checked, so the Russian lens is replaced; the present and future leave the axis open, which keeps the verdict at `fires` | Riegel, Pellat & Rioul, Grammaire méthodique du français (1994) | checked |
| French | K6 | fires | a possessive determiner (*son / sa / ses*), *dont*, or a *de*-genitive naming a possessor, part or kin: *l'erreur de sa vie* | *son* does not mark whether the possessor is the subject; *de* after a nominalisation, a quantity or a name is not possession and belongs to K8 | Riegel, Pellat & Rioul, Grammaire méthodique du français (1994) | checked |
| French | K7 | fires | *parce que*, *car*, *puisque*, *à cause de*, *grâce à*, *à la suite de*, *en raison de*, *être à l'origine de*, causal *pour* + noun, causal *par* | *puisque* presents the cause as already granted, so the causal claim is presupposed rather than asserted | Groupe λ-l, Car, parce que, puisque, Revue romane 10 (1975) | checked |
| French | K8 | fires | *N de N* or bare *N N* in any slot: *expert en politique*, *prix Nobel*, *zone euro* | *de* names that there is a relation and not which one; tentative: relational adjectives (*énergie lumineuse*) carry it as in Russian and Polish | Riegel, Pellat & Rioul, Grammaire méthodique du français (1994) | checked |
| Spanish | K1 | fires | any sentence | | RAE & ASALE, Nueva gramática de la lengua española (2009) | derived |
| Spanish | K2 | fires | impersonal, passive or anticausative *se*; the agentless *ser* passive or a bare participle (*fue condenada*); a non-actor or nominalisation as subject; the subjectless third plural (*nos echan*) | a null subject recoverable from the verb or the previous sentence is not the trigger | RAE & ASALE, Nueva gramática de la lengua española (2009) | checked |
| Spanish | K3 | fires | an inference, interpretation or borrowed claim in the plain indicative with no source named, where a conditional or a hedge was available | a named source (*según*, *dijo que*) already marks the report; the *condicional de rumor* did not occur in the 40 news sentences checked | RAE & ASALE, Nueva gramática de la lengua española (2009) | checked |
| Spanish | K4 | fires | a definite singular collective or institution in an agent slot (*la población*, *el Gobierno*), or a bare proper name of a state or firm acting as one (*España*) | the generic definite article of mass and abstract nouns is obligatory (*la inflación*, *la vida*) and does not by itself carry the trigger | RAE & ASALE, Nueva gramática de la lengua española (2009) | checked |
| Spanish | K5 | fires | pretérito perfecto or indefinido over a running state: *siempre ha funcionado*, *funcionó* | the past already splits indefinido from imperfecto, but the indefinido marks boundedness, not result; the pincer bites where the imperfecto or *estar* + gerund was available and not taken, and not over counted discrete events or the same-day news perfect | RAE & ASALE, Nueva gramática de la lengua española (2009) | checked |
| Spanish | K6 | fires | a possessive determiner of any person (*su*, *nuestro*, *mi*), *cuyo*, or a *de*-genitive naming a possessor | *su* never shows whether the possessor is the subject; *de* after an event noun, a place name, a partitive or a relational head (*aliado de*) is not the trigger | RAE & ASALE, Nueva gramática de la lengua española (2009) | checked |
| Spanish | K7 | fires | *porque*, *ya que*, *puesto que*, *dado que*, *debido a*, *gracias a*, causal *por* + noun or infinitive, *por tanto*, *por ello* | *ya que* and *puesto que* present the cause as given rather than asserted | RAE & ASALE, Nueva gramática de la lengua española (2009) | checked |
| Spanish | K8 | fires | *N de N*, *N de* + article + *N*, or bare *N N* in any slot: *experto en política*, *semana de la vivienda*, *pieza clave* | the preposition names that there is a relation and not which one; a deverbal head with its theme (*expulsión de migrantes*) is not the trigger | RAE & ASALE, Nueva gramática de la lengua española (2009) | checked |
| Russian | K1 | fires | any sentence | | Timberlake, A Reference Grammar of Russian (2004) | derived |
| Russian | K2 | fires | the indefinite-personal third plural, a *-ся* passive, an agentless passive participle (*был убит*), an actorless nominalisation (*задержание*), a non-agent as subject of an action verb, or an impersonal modal (*следует*, *нельзя*) | the third plural without a subject asserts a doer and refuses to name one | Timberlake, A Reference Grammar of Russian (2004) | checked |
| Russian | K3 | fires | borrowed or inferred content with no source named: no reportive particle (*якобы*, *мол*, *будто бы*) and no lexical attribution (*по словам*, *сообщает*) | the reportive particles did not occur in the 100 sentences checked; a named source already marks the report | Timberlake, A Reference Grammar of Russian (2004) | checked |
| Russian | K4 | fires | a specific collective or institution acting as one (*войско*, *власти*, *Кремль*), or a bare singular noun for a population: *рабочий* | there is no article; plain mass nouns (*энергия*) are not the trigger. Agreement with quantified subjects is free (*пришло* or *пришли пять человек*, also below five), so the Polish lens still forces a choice Russian leaves open | Timberlake, A Reference Grammar of Russian (2004) | checked |
| Russian | K5 | inverted | a perfective verb, participle or gerund over a process the facts leave open: *сработало*, *дал толчок* | aspect is obligatory, so a process cannot be left open; the pincer tests whether the completion the grammar forced was warranted (questionable in 6 of 11 cases checked) | Timberlake, A Reference Grammar of Russian (2004) | checked |
| Russian | K6 | fires | a possessive pronoun (including *свой*), a possessive adjective, a *у*-phrase or a possessor genitive | *свой* settles only whether the possessor is the subject, not the kind of possession; a genitive chain without possession belongs to K8 | Timberlake, A Reference Grammar of Russian (2004) | checked |
| Russian | K7 | fires | *потому что*, *так как*, *из-за*, *благодаря*, *вследствие*, *в результате*, *в связи с*, *так что*, causal *от* | *благодаря* adds a positive evaluation to the cause | Timberlake, A Reference Grammar of Russian (2004) | checked |
| Russian | K8 | fires | a relational adjective with its noun, or a bare adnominal genitive, in any slot: *плановая экономика*, *уровень цен* | Russian compounds rarely; the relational adjective and the genitive chain do the same work and hide the relation the same way. Russian predicate case follows the copula, not category against property, so the Polish lens stays a contrast | Timberlake, A Reference Grammar of Russian (2004) | checked |
| Polish | K1 | fires | any sentence | | Swan, Polish Grammar in a Nutshell (2003) | derived |
| Polish | K2 | fires | the impersonal *-no / -to* form, impersonal *się*, an agentless *być / zostać* + participle, or an inanimate subject of an action verb | the *-no / -to* form occurred in only 2 of 100 sentences checked; impersonal *się* and the agentless passive carried the trigger far more often | Swan, Polish Grammar in a Nutshell (2003) | checked |
| Polish | K3 | fires | borrowed content with no source named: no *podobno*, *rzekomo* or *jakoby*, and no verb of saying with a named speaker | a named speaker already marks the report; tentative: inferences about intentions stated without a hedge carried it too | Swan, Polish Grammar in a Nutshell (2003) | checked |
| Polish | K4 | fires | a singular collective or institution standing for its members (*zakon*, *wojsko*, *rząd*), or a bare singular noun for a population: *robotnik* | there is no article; plain mass readings (*tlen*, *herbata*) are not the trigger | Swan, Polish Grammar in a Nutshell (2003) | checked |
| Polish | K5 | inverted | a perfective verb over a process the facts leave open: *zadziałało*, *dał impuls* | aspect is obligatory, so a process cannot be left open; the pincer tests whether the completion the grammar forced was warranted. Tentative: perfective verbal nouns (*rozwinięcie*) carried it too | Swan, Polish Grammar in a Nutshell (2003) | checked |
| Polish | K6 | fires | a possessive pronoun, *swój*, *mieć* or a possessor genitive | *swój* settles only whether the possessor is the subject, not the kind of possession; a genitive chain without possession (*poziom cen*) belongs to K8 | Swan, Polish Grammar in a Nutshell (2003) | checked |
| Polish | K7 | fires | *bo*, *ponieważ*, *dlatego że*, *z powodu*, *więc*, *wskutek*, *dzięki*, *w efekcie*, *zwłaszcza że*, *za* + accusative of reason |  | Swan, Polish Grammar in a Nutshell (2003) | checked |
| Polish | K8 | fires | a relational adjective with its noun, or a noun with a bare genitive noun, in any slot: *gospodarka planowa*, *poziom cen*, *szef MSZ* | Polish compounds rarely; the relational adjective and the genitive chain do the same work and hide the relation the same way | Swan, Polish Grammar in a Nutshell (2003) | checked |
| Japanese | K1 | fires | any sentence | | Shibatani, The Languages of Japan (1990) | derived |
| Japanese | K2 | fires | a collective or non-actor subject (*kuni wa*, *kitei wa kinshi suru*), or a dropped subject the preceding sentence does not supply | a dropped subject recoverable from the previous sentence is ordinary zero anaphora, not the trigger | Shibatani, The Languages of Japan (1990) | checked |
| Japanese | K3 | fires | a plain assertion of borrowed or inferred content without *to sareru*, *to iwareru*, *to mirareru*, *ni yoreba* or an evidential ending (*sō da*, *rashii*, *yō da*) | the evidential endings did not occur in the 100 sentences checked; the quotative forms did, 12 times | Makino & Tsutsui, A Dictionary of Basic Japanese Grammar (1986) | checked |
| Japanese | K4 | fires | a bare noun for a population, mass or institution: *rōdōsha*, *shihonshugi*, *bakufu* | nouns carry neither article nor number, so any generic noun is a candidate; tentative: proper-named institutions, clans and states acting as one unit carried it too | Shibatani, The Languages of Japan (1990) | checked |
| Japanese | K5 | fires | *-ta*, a continuative or a verbal-noun ending over a process that ran for a span, where *-te kita*, *-yō ni natta* or *shidai ni* was available | *-te iru* occurred in 22 sentences checked and carried the trigger in none | Shibatani, The Languages of Japan (1990) | checked |
| Japanese | K6 | fires | belonging *no*, *motsu*, or *ni … ga aru* | *no* in general joins two nouns under any relation and belongs to K8; *jibun* marks the reflexive optionally only | Makino & Tsutsui, A Dictionary of Basic Japanese Grammar (1986) | checked |
| Japanese | K7 | fires | *kara*, *node*, *ni yotte*, *ni yori*, *koto de*, *koto kara*, *ni tomonai*, *o riyū ni* | *kara* presents the cause as the speaker's reason, *node* as an objective fact; *tame (ni)* meant purpose in every case checked | Makino & Tsutsui, A Dictionary of Basic Japanese Grammar (1986) | checked |
| Japanese | K8 | fires | a Sino-Japanese compound in any slot: *keikaku keizai*, *seiji senmonka* | compounds are dense and unmarked, the closest profile to English here; tentative: *N no N* carried it too | Shibatani, The Languages of Japan (1990) | checked |
| Chinese | K1 | fires | any sentence |  | Li & Thompson, Mandarin Chinese: A Functional Reference Grammar (1981) | derived |
| Chinese | K2 | fires | an agentless *被* passive (*被批评了*), a notional passive with the patient as topic and no agent (*房子盖好了*), impersonal *有人*, a dropped subject the preceding clause does not supply, or a non-actor as subject of an action verb: *研究表明*, *市场决定* | a dropped subject recoverable from the previous clause is ordinary zero anaphora, not the trigger; *被* often reads as adversative, which marks the event as unwelcome and still leaves the actor out; tentative: *受…影响* with a non-actor as the influence carried it too | Li & Thompson, Mandarin Chinese: A Functional Reference Grammar (1981) | checked |
| Chinese | K3 | fires | a plain assertion of borrowed or inferred content without *据说*, *据报道*, *据称*, *听说*, *被认为*, *看来*, *似乎*, *好像*, *(很)可能*, *有可能* or a named source (*据…称*, *…认为*, *…写道*) | there is no grammatical evidential; the source is marked only lexically, as in English. Plain exposition of definitions and present fact is not the trigger: the surface test alone fired on 90 of 100 sentences checked | Li & Thompson, Mandarin Chinese: A Functional Reference Grammar (1981) | checked |
| Chinese | K4 | fires | a noun for a population, mass or institution in any slot, possessor and object included, bare or with a demonstrative or group classifier: *工人*, *资本主义*, *政府*, *此类*, *这类*, *一群*; or a state acting as one unit: *中国* | nouns carry neither article nor number, and *们* is optional and confined to humans: it occurred 7 times in 100 sentences checked and never on a trigger noun; restricting the marker to agent and topic slots missed 16 of 59 triggers | Li & Thompson, Mandarin Chinese: A Functional Reference Grammar (1981) | checked |
| Chinese | K5 | fires | *了*, a resultative verb compound (*V完*, *V成*, *V好*) or a bare narrative verb (often with *彻底*, *不再*, a date or a period) over a process that ran for a span, where *在*, *着*, *一直*, *逐渐*, *越来越* or *V下来* / *V起来* was available | aspect particles exist but are optional, so completion is never forced: *了* or a resultative compound sat on the trigger verb in 9 of 18 trigger sentences checked, a bare verb in the rest; both sat on point acts (*爆发了*, *改为*) far more often, which are not the trigger | Li & Thompson, Mandarin Chinese: A Functional Reference Grammar (1981) | checked |
| Chinese | K6 | fires | possessive *的* after a pronoun or a possessor noun (*他的*, *公司的*), a possessor joined without *的* (*我妈妈*, *中国经济*), *其*, *有* with an owner as subject, or a kin or inalienable noun whose possessor is left to context: *父亲*, *家乡*, *原名* | *的* in general joins two nouns under any relation and belongs to K8; *他的* does not mark whether the possessor is the subject, and *自己* (0 of 100 checked) marks it optionally only; *的* was dropped for kin and institutions and as often for other compounds, so there is no alienability split and the Navajo lens stays | Li & Thompson, Mandarin Chinese: A Functional Reference Grammar (1981) | checked |
| Chinese | K7 | fires | *因为*, *因*, *由于*, *所以*, *故*, *因此*, *因而*, *于是*, *以致*, *结果*, *既然*, *鉴于*, *凭借*; *由…(而)引发*, *受…影响*; causal verbs *导致*, *造成*, *引起*, *使(得)* where one cause is named; causal *随着* | *既然* presents the cause as already granted, like *puisque*; *因* and *故* are Classical forms still current in written Mandarin; *随着* is temporal as often as causal, count it only where the reading is causal; clauses juxtaposed without a connective leave the cause to the reader and do not carry the trigger | Li & Thompson, Mandarin Chinese: A Functional Reference Grammar (1981) | checked |
| Chinese | K8 | fires | a noun–noun compound, a verb–object modifier with its noun (*便民服务*, *施政方式*) or *N 的 N* in any slot: *计划经济*, *政治专家*, *价格水平*, *经济的发展* | compounds are dense and unmarked, the closest profile to Japanese here; *的* names that there is a relation and not which one, and a *的*-phrase with a possessor belongs to K6 | Li & Thompson, Mandarin Chinese: A Functional Reference Grammar (1981) | checked |
| Arabic | K1 | fires | any sentence |  | Ryding, A Reference Grammar of Modern Standard Arabic (2005) | derived |
| Arabic | K2 | fires | the internal passive (*كُتِبَ*, *يُعتبر*), *تمّ* or *يجري* + verbal noun (*تمّ اعتقال*), a verbal noun whose actor is left out (*خلق الأرصدة*, *لا يمكن التهوين*), impersonal *يُقال*, an anticausative form VII or V (*انكسر*, *تطوّر*), or a non-actor as subject of an action verb: *تُظهر الدراسة*, *قرّر السوق* | the internal passive admits no agent in the classical norm, and *من قِبَل* adds one only in modern prose; a subject recoverable from the verb's agreement is not the trigger; the actorless verbal noun carried 8 of 18 misses checked; tentative: form VI (*تتزايد*) and passive participles (*المقصود*) carried it too | Ryding, A Reference Grammar of Modern Standard Arabic (2005) | checked |
| Arabic | K3 | fires | borrowed or inferred content in the plain indicative without *يُقال*, *يُعتقد*, *يُذكر*, *يبدو*, *ربّما*, *قد* + imperfect, or a named source (*وفقاً لـ*, *حسب*, *نقلاً عن*, *قال*, *أفاد*) | there is no grammatical evidential; *زعم* marks a report as doubted, *أكّد* as endorsed; plain exposition of documented fact is not the trigger: it made up 22 of 27 false hits checked | Ryding, A Reference Grammar of Modern Standard Arabic (2005) | checked |
| Arabic | K4 | fires | a definite singular collective, institution or generic noun for a population (*العامل*, *الشعب*, *الحكومة*, *الإنسان*), a collective noun without its unit form (*الشجر*, *النمل*), or a bare name of a state or city standing for its government: *إيران*, *دمشق* | the generic definite article of mass and abstract nouns is obligatory (*الاقتصاد*) and does not by itself carry the trigger, but a mass bounded by a demonstrative or quantifier does: *هذه الطاقة*, *معظم الطاقة*. The feminine singular agreement of non-human plurals is obligatory too and does not carry it either. The count/mass class settled the Yucatec question in 9 of 10 sentences checked; the lens stays, as for English and French, to test whether that settlement was warranted | Ryding, A Reference Grammar of Modern Standard Arabic (2005) | checked |
| Arabic | K5 | fires | a perfective verb over a running state or process, where *كان* + imperfect, *ظلّ*, *ما زال* or *أخذ / بدأ* + imperfect was available, or a verbal noun over a past process in narrative: *بعد انتشار شربها* | in the past the perfective against *كان* + imperfect already forces the perfective choice, so the Russian lens is replaced as for French (Russian forced no new choice in 8 of 10 checked); the imperfect covers the present without an aspect choice, which keeps the verdict at `fires`. A generic or definitional verbal noun (*ارتفاع الأسعار*) is not the trigger: it made up 32 of 35 false hits checked | Ryding, A Reference Grammar of Modern Standard Arabic (2005) | checked |
| Arabic | K6 | fires | a possessive pronoun suffix (*كتابه*), an *iḍāfa* with a possessor noun (*بيت أحمد*), possessive *لـ* (*كتاب لأحمد*, *له*), or *عند / لدى* | the suffix *-hu* does not mark whether the possessor is the subject, and *نفسه* (0 of 100 checked) marks it optionally only; an *iḍāfa* in general joins two nouns under any relation and belongs to K8; *لـ* of purpose or relation is not possession; tentative: a suffix that is the object of a verbal noun (*تنفيذه*) is not the trigger | Ryding, A Reference Grammar of Modern Standard Arabic (2005) | checked |
| Arabic | K7 | fires | *لأنّ*, *بسبب*, *نتيجةً لـ*, *بفضل*, *بفعل*, *إثر*, *جرّاء*, *على خلفية*, *نظراً لـ*, *بما أنّ*, *إذ*, causal *حيث*, *لذلك*, *لذا*, *بذلك*, causal *فـ*, causal *لـ* + verbal noun; means read as cause: *من خلال*, *بواسطة*, *بموجب*, instrumental *بـ* + verbal noun; causal verbs *أدّى إلى*, *تسبّب في* | *بما أنّ* presents the cause as already granted, like *puisque*; *فـ* is sequential as often as causal, count it only where the reading is causal; the means prepositions carried 7 of 18 misses checked; tentative: *مما*, *ناجم عن*, *نابع من* and causal *لـ* before any noun carried it too | Ryding, A Reference Grammar of Modern Standard Arabic (2005) | checked |
| Arabic | K8 | fires | an *iḍāfa* chain or a relational (*nisba*) adjective with its noun, in any slot: *مستوى الأسعار*, *خبير سياسي*, *النظام الاقتصادي* | the *iḍāfa* names that there is a relation and not which one; a verbal noun with its object (*طرد المهاجرين*) is not the trigger | Ryding, A Reference Grammar of Modern Standard Arabic (2005) | checked |

## Substitutions

`Replacement` empty means the lens is dropped without a successor: either no
sourced lens asks a different question on that axis, or the slot's question is
the one the source grammar already answers.

| Source language | Combination | Removed | Replacement | Reason |
|---|---|---|---|---|
| Russian | K5 | Russian | Hindi-Urdu (vector verbs) | own entry |
| Russian | K5 | Finnish (partitive object) | | Russian aspect already states how far the event reached the object, or the clause has none (9 of 10 checked) |
| Russian | K6 | Mongolian (reflexive possession) | | *свой* already forces whether a third-person possessor is the subject |
| French | K5 | Russian | Hindi-Urdu (vector verbs) | in the past the passé composé or simple against the imparfait already forces the perfective choice; Russian forced no new choice in 9 of 10 trigger sentences checked, Hindi-Urdu one in 9 of 10 |
| Polish | K4 | Polish (numeral threshold) | Hausa (pluractional verbs) | own entry |
| Polish | K5 | Russian | Hindi-Urdu (vector verbs) | the same binary aspect on this axis; kinship judged per axis, and here it is identity |
| Polish | K5 | Finnish (partitive object) | | Polish aspect, the partitive genitive and the genitive of negation already state how far the event reached the object (7 of 10 checked; the rest were intransitive) |
| Polish | K6 | Mongolian (reflexive possession) | | *swój* already forces whether a third-person possessor is the subject |
| Polish | K8 | Polish (instrumental predication) | | own entry; the grammar forces the category/property choice only at a copula, and the catalogue holds no sourced lens to replace it |
| German | K8 | Ancient Greek (article & substantivization) | | German turns adjectives, infinitives and clauses into article-marked nouns the same way (*das Erstarken*, *dem Bemühen*); in 10 of 10 trigger sentences checked every element was one already, so the lens forced no new choice. The slot's question is the one the source grammar already answers, so it is dropped rather than replaced |
| Spanish | K8 | Ancient Greek (article & substantivization) | | Spanish substantivises with the article, down to the neuter *lo* (*lo bueno*), and bare *de* against *de* + article already decided whether a type or a bounded thing was meant in 7 of 10 trigger sentences checked. The slot's question is the one the source grammar already answers, so it is dropped rather than replaced |
| Japanese | K4 | Yucatec Maya | Hausa (pluractional verbs) | Japanese, like Yucatec, leaves count and mass unmarked on the noun and counts through numeral classifiers, so the Yucatec lens forced no new choice (0 of 10 checked). Hausa forced one in 8 of 10; Spanish (differential object marking), the first replacement, found an object to attach to in 1 of 10, because the noun sits in subject or topic position |
| Chinese | K4 | Yucatec Maya | Hausa (pluractional verbs) | Mandarin, like Yucatec and Japanese, leaves count and mass unmarked on the noun and counts through numeral classifiers (*一个人*, *一张纸*), so the Yucatec lens forced no new choice (0 of 10 checked). Hausa forced one in 4 of 10, fewer than in Japanese, because verbal classifiers (*次*, *多次*, *第一次*) had already counted the event in the rest |
| Arabic | K5 | Russian | Hindi-Urdu (vector verbs) | in the past the perfective against *كان* + imperfect already forces the perfective choice, as the passé composé against the imparfait does in French; Russian forced no new choice in 8 of 10 trigger sentences checked (the two exceptions were durative states, *دام*, *عاش*), Hindi-Urdu one in 7 of 10 |
| Arabic | K5 | Finnish (partitive object) | | a telic perfective with a definite object already states that the event reached the whole object, and a partial reach takes *في* (*قرأ في الكتاب*); Finnish forced no new choice in any of 8 trigger sentences with an object checked |
| Arabic | K8 | Ancient Greek (article & substantivization) | | Arabic turns adjectives and participles into article-marked nouns the same way (*الجميل*, *المعروف*), and the verbal noun does so for any verb; the lens forced no new choice in 10 of 10 trigger sentences checked. The slot's question is the one the source grammar already answers, so it is dropped rather than replaced |

## Generating routing_seed.sql

```
python tools/gen_routing.py      # -> src/routing_seed.sql
python tools/check_doctrine.py   # the skill's rosters against this file
```

- Every lens, own entry and replacement must exist in `lat_catalog.md`, every
  lens and replacement must be `sourced`, and a lens must sit in the section
  its `Kind` names.
- Every profile must give a verdict for every combination; a missing row would
  read as "this combination does not apply" when nobody looked.
- An own entry of a profile that rides in a roster must be substituted for that
  profile, and a replacement may be neither an own entry nor already in the
  roster. Within each effective roster no two lenses may share a
  `forced_choice` at the same `attachment`.
- Profile names must differ by more than case, because the database matches
  them without regard to case. Only level-2 headings (`## `) delimit the
  tables; a deeper heading inside one is ignored, not treated as its end.
- Any violation aborts generation rather than reaching the database. The seed is
  applied when the routing tables are empty, independently of the pattern
  tables, so an existing database picks it up on the next start.
