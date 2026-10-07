-- SPDX-License-Identifier: Apache-2.0
-- Copyright 2026 Dana Schlifka
--
-- GENERATED from additional_docs/lat_routing.md by tools/gen_routing.py.
-- Do not edit by hand; edit the routing file and regenerate.
-- Applied on start when the routing tables are empty.
-- own_entries is a JSON array; an empty replacement means dropped.

INSERT INTO combinations
    (id, name, axis, trigger_text, provenance)
VALUES
    ('K1', 'base cycle', '—', 'any sentence', 'run: ten German sentences'),
    ('K2', 'agent pincer', 'Agency & control', 'a participant that cannot act stands where the actor goes, or the actor is left out', 'run: ten German sentences'),
    ('K3', 'source pincer', 'Evidence & certainty', 'inference, report or memory is asserted as observation', 'run: ten German sentences'),
    ('K4', 'boundary pincer', 'Object boundaries', 'a population, mass or bundle is named as one bounded thing', 'run: ten German sentences'),
    ('K5', 'aspect pincer', 'Time & aspect', 'a running process is presented as a completed point', 'run: ten German sentences'),
    ('K6', 'possession pincer', 'Possession & belonging', 'a possessive relation stands without its kind declared', 'run: ten German sentences'),
    ('K7', 'simultaneity pincer', 'Causality', 'a single cause is asserted by a connective or preposition', 'run: ten German sentences'),
    ('K8', 'compound pincer', 'Object boundaries', 'two nouns are joined without the relation between them named', 'run: one English text');

INSERT INTO combination_lenses
    (combination, position, name, kind, role, polarity)
VALUES
    ('K1', 1, 'Node collapse', 'form', 'reduction', 'constructive'),
    ('K1', 2, 'Word ban', 'form', 'ablation', 'both'),
    ('K1', 3, 'Back-translation', 'form', 'counter-check', 'destructive'),
    ('K1', 4, 'Form-switch', 'form', 'reconstruction', 'constructive'),
    ('K2', 1, 'Basque (Euskara)', 'language', 'ablation', 'both'),
    ('K2', 2, 'Nez Percé (Nimipuutímt)', 'language', 'ablation', 'destructive'),
    ('K2', 3, 'Tagalog', 'language', 'reconstruction', 'constructive'),
    ('K3', 1, 'Tuyuca', 'language', 'ablation', 'destructive'),
    ('K3', 2, 'Ewe (logophoric pronouns)', 'language', 'ablation', 'destructive'),
    ('K3', 3, 'Latin (oratio obliqua)', 'language', 'reconstruction', 'constructive'),
    ('K4', 1, 'Yucatec Maya', 'language', 'ablation', 'destructive'),
    ('K4', 2, 'Polish (numeral threshold)', 'language', 'ablation', 'destructive'),
    ('K4', 3, 'Hungarian (associative plural)', 'language', 'ablation', 'destructive'),
    ('K4', 4, 'Mojeño Trinitario (possessive classes)', 'language', 'ablation', 'destructive'),
    ('K4', 5, 'Toki Pona', 'language', 'reduction', 'both'),
    ('K4', 6, 'Swahili (noun classes)', 'language', 'reconstruction', 'constructive'),
    ('K5', 1, 'Russian', 'language', 'ablation', 'destructive'),
    ('K5', 2, 'Finnish (partitive object)', 'language', 'ablation', 'destructive'),
    ('K5', 3, 'Guaraní', 'language', 'reconstruction', 'constructive'),
    ('K6', 1, 'Navajo (inalienable possession)', 'language', 'ablation', 'destructive'),
    ('K6', 2, 'Hawaiian (a/o possession)', 'language', 'ablation', 'destructive'),
    ('K6', 3, 'Mongolian (reflexive possession)', 'language', 'ablation', 'destructive'),
    ('K6', 4, 'Pohnpeian (possessive classifiers)', 'language', 'reconstruction', 'constructive'),
    ('K7', 1, 'Fugue', 'form', 'ablation', 'both'),
    ('K7', 2, 'Latin (ablative absolute)', 'language', 'ablation', 'destructive'),
    ('K7', 3, 'Classical Chinese (Wényán)', 'language', 'ablation', 'both'),
    ('K7', 4, 'Dialectical Midrash', 'form', 'reconstruction', 'constructive'),
    ('K8', 1, 'Vigraha (compound resolution)', 'form', 'ablation', 'destructive'),
    ('K8', 2, 'Ancient Greek (article & substantivization)', 'language', 'ablation', 'destructive'),
    ('K8', 3, 'Polish (instrumental predication)', 'language', 'ablation', 'both'),
    ('K8', 4, 'Lojban', 'language', 'reconstruction', 'constructive');

INSERT INTO routing_profiles
    (source_language, own_entries, note)
VALUES
    ('German', '["German"]', 'the language the routing was first derived on'),
    ('English', '["English"]', 'K8 was derived on an English text'),
    ('French', '[]', 'the catalogue holds no French entry'),
    ('Spanish', '["Spanish (differential object marking)"]', ''),
    ('Russian', '["Russian"]', ''),
    ('Polish', '["Polish (instrumental predication)", "Polish (numeral threshold)", "Polish (masculine-personal plural)"]', ''),
    ('Japanese', '["Japanese", "Japanese (listing particles)"]', '');

INSERT INTO routing
    (source_language, combination, verdict, marker, note, citation, provenance)
VALUES
    ('German', 'K1', 'fires', 'any sentence', '', 'Hawkins, A Comparative Typology of English and German (1986)', 'run'),
    ('German', 'K2', 'fires', 'an agentless passive or its equivalents: *wurde gebaut*, *gilt als*, *ist zu tun*, *es wird*; or a non-actor as subject of an action verb: *die Analyse zeigt*, *der Markt regelt*', 'verbs of occurring (*entsteht*, *findet statt*, *steigt*) are not action verbs and do not carry the trigger', 'Hawkins, A Comparative Typology of English and German (1986)', 'checked'),
    ('German', 'K3', 'fires', 'inferred, evaluative or borrowed content in the indicative with no source named, no Konjunktiv I and no lexical hedge such as *gilt als*, *vermutlich*, *dürfte*, *überliefert*', 'plain exposition of documented fact is not the trigger; a named source or Konjunktiv I already marks a report', 'Duden, Die Grammatik (9th ed., 2016)', 'checked'),
    ('German', 'K4', 'fires', 'a singular noun phrase standing for a bundle, mass or population, definite or not: *der Arbeiter*, *die Wirtschaft*, *Ost und West*, *eine Gesellschaft*', '', 'Duden, Die Grammatik (9th ed., 2016)', 'checked'),
    ('German', 'K5', 'fires', 'Perfekt or Präteritum over a running process: *hat immer funktioniert*', 'aspect is not grammatical, so completion is never forced; tentative: prefixed verbs (*ent-*, *aus-*, *ver-*, *er-*) often mark it lexically, and a duration adjunct (*ab 1912*) blocks the marker', 'Hawkins, A Comparative Typology of English and German (1986)', 'checked'),
    ('German', 'K6', 'fires', 'a possessive pronoun, a possessor genitive or a possessive *von*-phrase: *seines Lebens*, *dessen Folgen*, *das Haus von Anna*', '*sein* does not mark whether the possessor is the subject, while *dessen / deren* marks a possessor that is not; a genitive that is the argument of a nominalisation is not possession and belongs to K8', 'Duden, Die Grammatik (9th ed., 2016)', 'checked'),
    ('German', 'K7', 'fires', '*weil*, *da*, *denn*; *deshalb*, *daher*, *folglich*, *infolgedessen*, *nämlich*; causal *durch*, *wegen*, *aufgrund*, *infolge*, *angesichts*, *dank*; *daran, dass*', '*nach* and *seit* were temporal in every case checked; count them only where the reading is causal', 'Hawkins, A Comparative Typology of English and German (1986)', 'checked'),
    ('German', 'K8', 'fires', 'a noun–noun compound in any slot: *Planwirtschaft*, *Politikexperte*', 'German writes the compound as one word, which keeps it legible as a compound; restricting the marker to argument slots missed 12 of 61 cases', 'Duden, Die Grammatik (9th ed., 2016)', 'checked'),
    ('English', 'K1', 'fires', 'any sentence', '', 'Huddleston & Pullum, The Cambridge Grammar of the English Language (2002)', 'derived'),
    ('English', 'K2', 'fires', 'an agentless passive, or a non-human noun as subject of an action verb: *was chosen*, *the analysis shows*, *jazz drew on*', 'English nominalises freely, so more abstractions reach the subject slot without any morphology', 'Huddleston & Pullum, The Cambridge Grammar of the English Language (2002)', 'checked'),
    ('English', 'K3', 'fires', 'unsourced indicative over content nobody observed: a motive or inner state, a causal attribution, an evaluation or priority claim', 'English marks the source only lexically (*says*, *seems*, *allegedly*); a named source already marks the report', 'Huddleston & Pullum, The Cambridge Grammar of the English Language (2002)', 'checked'),
    ('English', 'K4', 'fires', 'a definite singular collective or a bare singular mass or genre noun: *the court*, *the public*, *jazz*', 'the bare plural carried the trigger in none of 52 cases checked; the earlier claim that the trigger inverts against German does not hold. The article settled count against mass in 10 of 10 sentences checked, and Yucatec Maya stays: it tests whether that settlement was warranted, as in German', 'Huddleston & Pullum, The Cambridge Grammar of the English Language (2002)', 'checked'),
    ('English', 'K5', 'fires', 'simple past or present perfect of a change verb over a span, where the progressive was available: *emerged in the 1940s*, *has always worked*', 'the progressive was available and unused in all 16 trigger sentences checked', 'Huddleston & Pullum, The Cambridge Grammar of the English Language (2002)', 'checked'),
    ('English', 'K6', 'fires', 'possessive *''s*, a possessive determiner (*its*, *their*, *his*), or an *of*-phrase naming a part, kin or property of its noun: *the mistake of his life*', '*his* leaves open whether the possessor is the subject; partitive, title and event *of*-phrases are not possession', 'Huddleston & Pullum, The Cambridge Grammar of the English Language (2002)', 'checked'),
    ('English', 'K7', 'fires', '*because*, *since*, *as*, *given*, *thus*, *consequently*, and causal prepositions: *because of*, *due to*, *out of*, *for*, *over*', 'a cause often hides in a noun phrase instead of opening a clause, and the word ban has to target the preposition then', 'Huddleston & Pullum, The Cambridge Grammar of the English Language (2002)', 'checked'),
    ('English', 'K8', 'fires', 'a noun compound of two or more nouns in any slot: *group differences*, *stereotype threat*', 'the pair reads as modifier and noun, and the case relation between the members vanishes without trace; the earlier marker, two nouns in an argument slot, missed 5 compounds in adjuncts and 5 stacks of three or more nouns or coordinated compounds', 'Huddleston & Pullum, The Cambridge Grammar of the English Language (2002)', 'checked'),
    ('French', 'K1', 'fires', 'any sentence', '', 'Riegel, Pellat & Rioul, Grammaire méthodique du français (1994)', 'derived'),
    ('French', 'K2', 'fires', 'an agentless passive (*être* + participle without *par*, or a bare participle), the pronominal passive (*s''opère*), *on*, or a non-actor as subject: *l''analyse révèle*, *cette évolution suscite*', '*on* occurred in only 2 of 100 sentences checked; a collective that really acts (*la France*, *la presse*) is a K4 case, not a K2 one', 'Riegel, Pellat & Rioul, Grammaire méthodique du français (1994)', 'checked'),
    ('French', 'K3', 'fires', 'inferred or borrowed content in the indicative with no source named, where the conditional (*journalistique* or of conjecture: *serait apparue*) was available', 'a named source (*selon*, *a annoncé*) already marks the report; the conditional did not occur in the 100 sentences checked', 'Dendale, Le conditionnel de l''information incertaine (1993)', 'checked'),
    ('French', 'K4', 'fires', 'definite singular for a bundle or population: *l''ouvrier*, *le capitalisme*', 'generic mass nouns took the definite singular rather than the partitive in the sentences checked; tentative: resuming pronouns (*celle-ci*) and indefinite collectives (*une série de*) carried it too. The article settled count against mass in 10 of 10 sentences checked, and Yucatec Maya stays: it tests whether that settlement was warranted, as in German', 'Riegel, Pellat & Rioul, Grammaire méthodique du français (1994)', 'checked'),
    ('French', 'K5', 'fires', 'passé composé over a running state: *a toujours bien fonctionné*', 'the past already splits perfective from imparfait, and the passé composé encoded completion in 9 of 10 trigger sentences checked, so the Russian lens is replaced; the present and future leave the axis open, which keeps the verdict at `fires`', 'Riegel, Pellat & Rioul, Grammaire méthodique du français (1994)', 'checked'),
    ('French', 'K6', 'fires', 'a possessive determiner (*son / sa / ses*), *dont*, or a *de*-genitive naming a possessor, part or kin: *l''erreur de sa vie*', '*son* does not mark whether the possessor is the subject; *de* after a nominalisation, a quantity or a name is not possession and belongs to K8', 'Riegel, Pellat & Rioul, Grammaire méthodique du français (1994)', 'checked'),
    ('French', 'K7', 'fires', '*parce que*, *car*, *puisque*, *à cause de*, *grâce à*, *à la suite de*, *en raison de*, *être à l''origine de*, causal *pour* + noun, causal *par*', '*puisque* presents the cause as already granted, so the causal claim is presupposed rather than asserted', 'Groupe λ-l, Car, parce que, puisque, Revue romane 10 (1975)', 'checked'),
    ('French', 'K8', 'fires', '*N de N* or bare *N N* in any slot: *expert en politique*, *prix Nobel*, *zone euro*', '*de* names that there is a relation and not which one; tentative: relational adjectives (*énergie lumineuse*) carry it as in Russian and Polish', 'Riegel, Pellat & Rioul, Grammaire méthodique du français (1994)', 'checked'),
    ('Spanish', 'K1', 'fires', 'any sentence', '', 'RAE & ASALE, Nueva gramática de la lengua española (2009)', 'derived'),
    ('Spanish', 'K2', 'fires', 'impersonal, passive or anticausative *se*; the agentless *ser* passive or a bare participle (*fue condenada*); a non-actor or nominalisation as subject; the subjectless third plural (*nos echan*)', 'a null subject recoverable from the verb or the previous sentence is not the trigger', 'RAE & ASALE, Nueva gramática de la lengua española (2009)', 'checked'),
    ('Spanish', 'K3', 'fires', 'an inference, interpretation or borrowed claim in the plain indicative with no source named, where a conditional or a hedge was available', 'a named source (*según*, *dijo que*) already marks the report; the *condicional de rumor* did not occur in the 40 news sentences checked', 'RAE & ASALE, Nueva gramática de la lengua española (2009)', 'checked'),
    ('Spanish', 'K4', 'fires', 'a definite singular collective or institution in an agent slot (*la población*, *el Gobierno*), or a bare proper name of a state or firm acting as one (*España*)', 'the generic definite article of mass and abstract nouns is obligatory (*la inflación*, *la vida*) and does not by itself carry the trigger', 'RAE & ASALE, Nueva gramática de la lengua española (2009)', 'checked'),
    ('Spanish', 'K5', 'fires', 'pretérito perfecto or indefinido over a running state: *siempre ha funcionado*, *funcionó*', 'the past already splits indefinido from imperfecto, but the indefinido marks boundedness, not result; the pincer bites where the imperfecto or *estar* + gerund was available and not taken, and not over counted discrete events or the same-day news perfect', 'RAE & ASALE, Nueva gramática de la lengua española (2009)', 'checked'),
    ('Spanish', 'K6', 'fires', 'a possessive determiner of any person (*su*, *nuestro*, *mi*), *cuyo*, or a *de*-genitive naming a possessor', '*su* never shows whether the possessor is the subject; *de* after an event noun, a place name, a partitive or a relational head (*aliado de*) is not the trigger', 'RAE & ASALE, Nueva gramática de la lengua española (2009)', 'checked'),
    ('Spanish', 'K7', 'fires', '*porque*, *ya que*, *puesto que*, *dado que*, *debido a*, *gracias a*, causal *por* + noun or infinitive, *por tanto*, *por ello*', '*ya que* and *puesto que* present the cause as given rather than asserted', 'RAE & ASALE, Nueva gramática de la lengua española (2009)', 'checked'),
    ('Spanish', 'K8', 'fires', '*N de N*, *N de* + article + *N*, or bare *N N* in any slot: *experto en política*, *semana de la vivienda*, *pieza clave*', 'the preposition names that there is a relation and not which one; a deverbal head with its theme (*expulsión de migrantes*) is not the trigger', 'RAE & ASALE, Nueva gramática de la lengua española (2009)', 'checked'),
    ('Russian', 'K1', 'fires', 'any sentence', '', 'Timberlake, A Reference Grammar of Russian (2004)', 'derived'),
    ('Russian', 'K2', 'fires', 'the indefinite-personal third plural, a *-ся* passive, an agentless passive participle (*был убит*), an actorless nominalisation (*задержание*), a non-agent as subject of an action verb, or an impersonal modal (*следует*, *нельзя*)', 'the third plural without a subject asserts a doer and refuses to name one', 'Timberlake, A Reference Grammar of Russian (2004)', 'checked'),
    ('Russian', 'K3', 'fires', 'borrowed or inferred content with no source named: no reportive particle (*якобы*, *мол*, *будто бы*) and no lexical attribution (*по словам*, *сообщает*)', 'the reportive particles did not occur in the 100 sentences checked; a named source already marks the report', 'Timberlake, A Reference Grammar of Russian (2004)', 'checked'),
    ('Russian', 'K4', 'fires', 'a specific collective or institution acting as one (*войско*, *власти*, *Кремль*), or a bare singular noun for a population: *рабочий*', 'there is no article; plain mass nouns (*энергия*) are not the trigger. Agreement with quantified subjects is free (*пришло* or *пришли пять человек*, also below five), so the Polish lens still forces a choice Russian leaves open', 'Timberlake, A Reference Grammar of Russian (2004)', 'checked'),
    ('Russian', 'K5', 'inverted', 'a perfective verb, participle or gerund over a process the facts leave open: *сработало*, *дал толчок*', 'aspect is obligatory, so a process cannot be left open; the pincer tests whether the completion the grammar forced was warranted (questionable in 6 of 11 cases checked)', 'Timberlake, A Reference Grammar of Russian (2004)', 'checked'),
    ('Russian', 'K6', 'fires', 'a possessive pronoun (including *свой*), a possessive adjective, a *у*-phrase or a possessor genitive', '*свой* settles only whether the possessor is the subject, not the kind of possession; a genitive chain without possession belongs to K8', 'Timberlake, A Reference Grammar of Russian (2004)', 'checked'),
    ('Russian', 'K7', 'fires', '*потому что*, *так как*, *из-за*, *благодаря*, *вследствие*, *в результате*, *в связи с*, *так что*, causal *от*', '*благодаря* adds a positive evaluation to the cause', 'Timberlake, A Reference Grammar of Russian (2004)', 'checked'),
    ('Russian', 'K8', 'fires', 'a relational adjective with its noun, or a bare adnominal genitive, in any slot: *плановая экономика*, *уровень цен*', 'Russian compounds rarely; the relational adjective and the genitive chain do the same work and hide the relation the same way. Russian predicate case follows the copula, not category against property, so the Polish lens stays a contrast', 'Timberlake, A Reference Grammar of Russian (2004)', 'checked'),
    ('Polish', 'K1', 'fires', 'any sentence', '', 'Swan, Polish Grammar in a Nutshell (2003)', 'derived'),
    ('Polish', 'K2', 'fires', 'the impersonal *-no / -to* form, impersonal *się*, an agentless *być / zostać* + participle, or an inanimate subject of an action verb', 'the *-no / -to* form occurred in only 2 of 100 sentences checked; impersonal *się* and the agentless passive carried the trigger far more often', 'Swan, Polish Grammar in a Nutshell (2003)', 'checked'),
    ('Polish', 'K3', 'fires', 'borrowed content with no source named: no *podobno*, *rzekomo* or *jakoby*, and no verb of saying with a named speaker', 'a named speaker already marks the report; tentative: inferences about intentions stated without a hedge carried it too', 'Swan, Polish Grammar in a Nutshell (2003)', 'checked'),
    ('Polish', 'K4', 'fires', 'a singular collective or institution standing for its members (*zakon*, *wojsko*, *rząd*), or a bare singular noun for a population: *robotnik*', 'there is no article; plain mass readings (*tlen*, *herbata*) are not the trigger', 'Swan, Polish Grammar in a Nutshell (2003)', 'checked'),
    ('Polish', 'K5', 'inverted', 'a perfective verb over a process the facts leave open: *zadziałało*, *dał impuls*', 'aspect is obligatory, so a process cannot be left open; the pincer tests whether the completion the grammar forced was warranted. Tentative: perfective verbal nouns (*rozwinięcie*) carried it too', 'Swan, Polish Grammar in a Nutshell (2003)', 'checked'),
    ('Polish', 'K6', 'fires', 'a possessive pronoun, *swój*, *mieć* or a possessor genitive', '*swój* settles only whether the possessor is the subject, not the kind of possession; a genitive chain without possession (*poziom cen*) belongs to K8', 'Swan, Polish Grammar in a Nutshell (2003)', 'checked'),
    ('Polish', 'K7', 'fires', '*bo*, *ponieważ*, *dlatego że*, *z powodu*, *więc*, *wskutek*, *dzięki*, *w efekcie*, *zwłaszcza że*, *za* + accusative of reason', '', 'Swan, Polish Grammar in a Nutshell (2003)', 'checked'),
    ('Polish', 'K8', 'fires', 'a relational adjective with its noun, or a noun with a bare genitive noun, in any slot: *gospodarka planowa*, *poziom cen*, *szef MSZ*', 'Polish compounds rarely; the relational adjective and the genitive chain do the same work and hide the relation the same way', 'Swan, Polish Grammar in a Nutshell (2003)', 'checked'),
    ('Japanese', 'K1', 'fires', 'any sentence', '', 'Shibatani, The Languages of Japan (1990)', 'derived'),
    ('Japanese', 'K2', 'fires', 'a collective or non-actor subject (*kuni wa*, *kitei wa kinshi suru*), or a dropped subject the preceding sentence does not supply', 'a dropped subject recoverable from the previous sentence is ordinary zero anaphora, not the trigger', 'Shibatani, The Languages of Japan (1990)', 'checked'),
    ('Japanese', 'K3', 'fires', 'a plain assertion of borrowed or inferred content without *to sareru*, *to iwareru*, *to mirareru*, *ni yoreba* or an evidential ending (*sō da*, *rashii*, *yō da*)', 'the evidential endings did not occur in the 100 sentences checked; the quotative forms did, 12 times', 'Makino & Tsutsui, A Dictionary of Basic Japanese Grammar (1986)', 'checked'),
    ('Japanese', 'K4', 'fires', 'a bare noun for a population, mass or institution: *rōdōsha*, *shihonshugi*, *bakufu*', 'nouns carry neither article nor number, so any generic noun is a candidate; tentative: proper-named institutions, clans and states acting as one unit carried it too', 'Shibatani, The Languages of Japan (1990)', 'checked'),
    ('Japanese', 'K5', 'fires', '*-ta*, a continuative or a verbal-noun ending over a process that ran for a span, where *-te kita*, *-yō ni natta* or *shidai ni* was available', '*-te iru* occurred in 22 sentences checked and carried the trigger in none', 'Shibatani, The Languages of Japan (1990)', 'checked'),
    ('Japanese', 'K6', 'fires', 'belonging *no*, *motsu*, or *ni … ga aru*', '*no* in general joins two nouns under any relation and belongs to K8; *jibun* marks the reflexive optionally only', 'Makino & Tsutsui, A Dictionary of Basic Japanese Grammar (1986)', 'checked'),
    ('Japanese', 'K7', 'fires', '*kara*, *node*, *ni yotte*, *ni yori*, *koto de*, *koto kara*, *ni tomonai*, *o riyū ni*', '*kara* presents the cause as the speaker''s reason, *node* as an objective fact; *tame (ni)* meant purpose in every case checked', 'Makino & Tsutsui, A Dictionary of Basic Japanese Grammar (1986)', 'checked'),
    ('Japanese', 'K8', 'fires', 'a Sino-Japanese compound in any slot: *keikaku keizai*, *seiji senmonka*', 'compounds are dense and unmarked, the closest profile to English here; tentative: *N no N* carried it too', 'Shibatani, The Languages of Japan (1990)', 'checked');

INSERT INTO substitutions
    (source_language, combination, removed, replacement, replacement_kind, reason)
VALUES
    ('Russian', 'K5', 'Russian', 'Hindi-Urdu (vector verbs)', 'language', 'own entry'),
    ('Russian', 'K5', 'Finnish (partitive object)', '', '', 'Russian aspect already states how far the event reached the object, or the clause has none (9 of 10 checked)'),
    ('Russian', 'K6', 'Mongolian (reflexive possession)', '', '', '*свой* already forces whether a third-person possessor is the subject'),
    ('French', 'K5', 'Russian', 'Hindi-Urdu (vector verbs)', 'language', 'in the past the passé composé or simple against the imparfait already forces the perfective choice; Russian forced no new choice in 9 of 10 trigger sentences checked, Hindi-Urdu one in 9 of 10'),
    ('Polish', 'K4', 'Polish (numeral threshold)', 'Hausa (pluractional verbs)', 'language', 'own entry'),
    ('Polish', 'K5', 'Russian', 'Hindi-Urdu (vector verbs)', 'language', 'the same binary aspect on this axis; kinship judged per axis, and here it is identity'),
    ('Polish', 'K5', 'Finnish (partitive object)', '', '', 'Polish aspect, the partitive genitive and the genitive of negation already state how far the event reached the object (7 of 10 checked; the rest were intransitive)'),
    ('Polish', 'K6', 'Mongolian (reflexive possession)', '', '', '*swój* already forces whether a third-person possessor is the subject'),
    ('Polish', 'K8', 'Polish (instrumental predication)', '', '', 'own entry; the grammar forces the category/property choice only at a copula, and the catalogue holds no sourced lens to replace it'),
    ('German', 'K8', 'Ancient Greek (article & substantivization)', '', '', 'German turns adjectives, infinitives and clauses into article-marked nouns the same way (*das Erstarken*, *dem Bemühen*); in 10 of 10 trigger sentences checked every element was one already, so the lens forced no new choice. The slot''s question is the one the source grammar already answers, so it is dropped rather than replaced'),
    ('Spanish', 'K8', 'Ancient Greek (article & substantivization)', '', '', 'Spanish substantivises with the article, down to the neuter *lo* (*lo bueno*), and bare *de* against *de* + article already decided whether a type or a bounded thing was meant in 7 of 10 trigger sentences checked. The slot''s question is the one the source grammar already answers, so it is dropped rather than replaced'),
    ('Japanese', 'K4', 'Yucatec Maya', 'Hausa (pluractional verbs)', 'language', 'Japanese, like Yucatec, leaves count and mass unmarked on the noun and counts through numeral classifiers, so the Yucatec lens forced no new choice (0 of 10 checked). Hausa forced one in 8 of 10; Spanish (differential object marking), the first replacement, found an object to attach to in 1 of 10, because the noun sits in subject or topic position');
