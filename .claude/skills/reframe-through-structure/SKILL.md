---
name: reframe-through-structure
description: >-
  When someone is stuck on a text, argument, or thought and wants to see it
  differently, reformulate it through the structure of another language or a
  bound poetic form to surface hidden assumptions — causal bias, agent-object
  hierarchy, tacit object boundaries. Uses the `lat` MCP as the pattern
  catalogue. Triggers include: "think about this differently", "reframe this
  paragraph", "another perspective on this thought", "language as a tool",
  back-translation, form-switch, word ban, perspective swap, "why does this
  sound so inevitable/causal", combining several lenses without interference.
---

# Reframe through structure

Languages and bound forms make certain distinctions obligatory. English and
German, for instance, lean reflexively toward the causal chain (subject acts,
object suffers, subordinate clause justifies) — an order that often belongs more
to the grammar than to the subject matter. This skill uses foreign structures as
a **change of lighting**: the same object, different shadows. The goal is not
the "correct" language but **mobility** — the moment it becomes clear that a
difficulty sat in the form of telling, not in the problem itself.

Background: the founding essay *Language as a Tool of Thought*, deposited on
Zenodo — https://doi.org/10.5281/zenodo.21382455

## When to apply

- A paragraph or argument sounds more inevitable than the facts warrant.
- Something is really simultaneous, reciprocal, or field-like but is being told
  as cause → effect.
- The user wants to "turn" a thought, see it freshly, or unstick a phrasing.
- Explicitly requested techniques: back-translation, form-switch, word ban,
  perspective swap.

Do not apply when a plain factual answer or rewrite without a perspective shift
is wanted.

## Prerequisite: the `lat` MCP

The pattern catalogue comes from the `lat` MCP server. Available tools:

- `mcp__lat__list_facets` — which themes / categories / classifications / tags exist.
- `mcp__lat__search_patterns` — find patterns by filters (`kind`, `theme`, `category`,
  `classification`, `focus`, `tag`, `text`, `exclude_names`; all optional,
  combined with AND).
- `mcp__lat__get_pattern` — full details of a pattern (`kind` + `name`).
- `mcp__lat__list_patterns` — overview.

**Always ground the choice of pattern in the catalogue** — never invent
languages or forms. If the MCP is not connected, say so and offer to work
provisionally with the example patterns documented in the repo (Russian,
Japanese, Yucatec Maya, English; Minnesang, Haiku, Enumeration; the techniques).

## Workflow

1. **Grasp the subject.** Get the concrete text/sentence/thought. If none is
   given, ask for a passage or a clear core statement.
2. **Diagnose the bias.** Name the quiet structure of the source phrasing:
   causal chain? agent-object rank? a decision about what counts as a "thing"?
   a fixed tense? This determines which theme to search for.
3. **Query the catalogue.** `theme` uses a closed vocabulary of cognitive axes;
   the diagnosed bias maps to one of them: `Causality`, `Agency & control`,
   `Rank & salience`, `Time & aspect`, `Coexistence`,
   `Perspective & reciprocity`, `Object boundaries`, `Evidence & certainty`,
   `Space & orientation`, `Possession & belonging`, `Logic & ambiguity`.
   `Agency & control` is about the actor's relation to the act (does one exist,
   was it willed, who caused it); `Rank & salience` about which participant
   stands in front of the others. Call `mcp__lat__list_facets` if unsure
   of the exact strings, then `mcp__lat__search_patterns` on that `theme`. Pull details
   with `mcp__lat__get_pattern` as needed (focus, feature). Search forms by
   `theme` as well — most carry one. Only patterns whose mechanic is purely
   acoustic, metrical or typographic carry none; reach those via
   `tag`/`category`/`text`. Two further filters serve combination rather than
   selection: `attachment` (exact, eleven anchors) and `forced_choice`
   (substring) — use them to check what a candidate lens would duplicate.
4. **Account for the input language (contrast, not match).** See the section
   below: exclude the user's own language and prefer language lenses that
   foreground what it backgrounds. Forms are language-neutral and always apply.
5. **Choose a technique** (see below) matching the pattern.
6. **Reformulate.** Force the subject through the structure — one, better
   several, contrasts, so that mobility arises. From two lenses on, run them as
   a combination (see below): every lens reads the source text in its original
   state, and the findings stay separate rows.
7. **Name what it reveals.** For each reformulation, state explicitly *what* it
   makes visible that the source phrasing had concealed. This is the real
   payoff, not the elegant wording.
8. **Frame it.** Make clear that no version is "more correct". The subject does
   not change, only the access to it.

## Input language: contrast, not match

A language lens is valuable in proportion to how *differently* it treats the
biased axis compared with the language of the analysed text. Russian aspect is a
revelation for a text written in English or German (where aspect is not
obligatory) but banal in a language that already forces it. Note the comparison
is per axis, not per language: two related languages can be identical on one
axis and opposed on the next. So:

- **Detect the input language** — the language of the *text being analysed*, not
  the language of the request. If the user writes to you in English about a
  German document, the input language is German. No tool needed; you read it
  directly.
- **Exclude it, and only it.** Pass `exclude_names: ["<that language>"]` to
  `mcp__lat__search_patterns` so the structure the text already thinks in is not
  recommended back. Exclude exactly that one language — never a pair, a family,
  or a list of relatives. `exclude_names` matches by **exact name only**, which
  is precisely what is wanted here.
- **A closely related language is still a valid lens.** Kinship is never uniform
  across the axes, so do not down-weight relatives wholesale. English and German
  do share the causal-chain / agent-patient reflex — but they diverge sharply
  elsewhere: English parataxis carries **coexistence** where German hypotaxis
  subordinates, and English lacks the case marking German leans on. For a German
  text English therefore stays available, and is the right lens exactly on the
  axes where the two differ. Judge per axis, not per family.
- **Prefer complementarity.** The best lens foregrounds (makes obligatory) the
  very axis the user's language leaves implicit. A German/English speaker stuck
  on causality gains most from aspect (Russian), evidentiality (Tuyuca), animacy
  (Ojibwe), or absolute space (Guugu Yimithirr) — axes their language backgrounds.
- **Forms are language-neutral.** A Haiku, a word ban, a Sestina works in any
  language, so form/technique choices do not depend on the input language; only
  the *language* lenses do.

If the input language is itself in the catalogue, look it up (its `focus` and
`themes` describe what it foregrounds) to reason about its blind spots; if not,
infer its rough profile. For a multilingual text, take the language the passage
under analysis is actually written in; if the catalogue has no entry under that
exact name, pass nothing and reason about the blind spots in prose instead.

## Techniques

Techniques are catalogue entries like any other: `kind: "form"`,
`category: "Technique"`. The catalogue is authoritative and grows — reach the
current set with
`mcp__lat__search_patterns { "kind": "form", "category": "Technique" }`
instead of working from a fixed list, and pull `focus` and `feature` with
`mcp__lat__get_pattern` before applying one.

The four procedural handles the triggers above name directly:

| Technique | What it does | Fits patterns whose focus is … |
|---|---|---|
| **Back-translation** | Translate a passage into a language and back; what breaks was language-bound. | robustness check; any language |
| **Form-switch** | Force the content into a foreign genre (a proof as a recipe, a process as a map). | forms: Minnesang, Haiku, Enumeration |
| **Word ban** | Drop a word for a paragraph (*because*, *leads to*, any directional verb). | causality, hierarchy |
| **Perspective swap** | Recount from the viewpoint of the supposed object (Japanese *wa*). | perspective, agent/object rank |

Further techniques attack one named structural property rather than prescribing
a procedure; choose them by the property, not by the trigger word:

| Technique | Attacks | Reach for it when … |
|---|---|---|
| **Scale test** | a claimed self-similarity | one description is asserted to hold at every level of a system |
| **Node collapse** | things as against relations | nouns carry the argument and it is unclear what in it is more than the relations asserted |
| **Vigraha (compound resolution)** | the silent relation inside a compound | a compound occupies an argument slot where a clause with named participants belongs |

The bound procedures of the antique and Oulipo traditions are filed as
techniques too: `Incantatio` (`Agency & control`), `Cento`
(`Perspective & reciprocity`) and `Clinamen` (`Logic & ambiguity`) are reachable
by `theme`; `Lipogram` and `S+7 (N+7)` are purely procedural, carry no theme,
and are reached via `category` or `tag` — as are `Back-translation` and
`Form-switch` above.

Language patterns supply the *grammatical lens*, form patterns the *genre lens*,
techniques the *handle*. Combine them: e.g. a perspective swap with the Japanese
topic structure, or a ban on "because" with Enumeration.

## Combining lenses

Several lenses on one text are worth more than one only if their findings stay
separable. The failure mode is invisible in the output: a distorted matrix
reads exactly like a clean one. Three kinds of interference produce it, and
only the first is about the text.

**Textual interference.** A lens reads the previous lens's rewriting instead of
the source. Drift accumulates, and every later finding holds for a text nobody
wrote. Guard: **every lens reads the source text in its original state.** What
travels between lenses is never a rewriting, only a scalar parameter — which
token, which span, which axis. `Word ban` has to know *which* word to drop; it
receives that one token from `Node collapse` and still reads the original
sentence. Sequence therefore constrains *what a lens is told*, never *what it
reads*.

**Redundancy interference.** Two lenses that force the same choice yield
correlated rows that look like two independent pieces of evidence. Tuyuca and
Tariana distort no text — they distort the count, and the weaker of the two
goes invisible behind the stronger. Guard: **no two lenses with the same forced
choice at the same attachment point** — the screen below. A shared axis is not
by itself a collision: the axis is a search bucket, eleven of them over the
whole catalogue, and K4 to K6 each stack four lenses on one axis on purpose.

**Role interference.** A constructive lens running among the diagnostic ones
turns its own replacement phrasing into the object of the later diagnoses.
Guard: **the reconstruction is strictly last** and stands outside the matrix.

### When two lenses may share an axis

The axis was built for *finding* a lens, not for excluding one. Two lenses
collide when they force **the same choice** at **the same attachment point**.
Both are catalogue fields, so this is a comparison, not a judgement call: every
pattern carries `forced_choice` (the choice it makes obligatory) and
`attachment` (the constituent it interrogates, from a closed list of eleven).
`forced_choice` is written to be *shared* — two patterns forcing the same
choice carry the identical string — which is exactly what `focus` cannot do,
being authored per entry to read distinctively.

| | `Yucatec Maya` | `Finnish (partitive object)` |
|---|---|---|
| `themes` | Object boundaries | Object boundaries, Time & aspect |
| `forced_choice` | whether the noun denotes a countable unit | how far the event reached the object |
| `attachment` | `noun` | `object` |

Same axis, two strings, two anchors: keep both. Against that, `Tuyuca` and
`Tariana` both read `the source of the information` at `verb` — one string,
one anchor, one finding.

To see every partner of a lens before combining, hand both fields back to the
catalogue:
`mcp__lat__search_patterns { "attachment": "verb", "forced_choice": "the source of the information" }`.
Anything that comes back beside the lens in hand is a lens not to add.

Where choice and anchor do coincide, the collision is of one of three kinds:

| Kind | Same choice and anchor, differing in | Keep |
|---|---|---|
| **graded** | the strictness of the threshold | the sharper one |
| **parallel** | only the mechanism; the verdict is the same | either, never both |
| **role collision** | nothing — two lenses filling one role | the one the role names |

`Nez Percé (Nimipuutímt)` and `Oromo (marked nominative)` are parallel: a
tripartite case system and a marked nominative, both settling whether the
participant is the one acting. `Node collapse`, `Toki Pona` and `Pirahã` are a
role collision rather than an axis one — three reducers, one slot, and their
`forced_choice` strings differ, so the screen does not catch them.

K6 passes the screen with four lenses on a single anchor: `possessive` carries
all four, but alienability, direction of dependence, anchor retention and
intended use are four different forced choices.

**The screen is cheap and incomplete.** `Basque (Euskara)` asks
`whether the act was willed`, `Nez Percé (Nimipuutímt)` asks
`whether the participant is the one acting` — two strings at the same anchor,
so both pass, and yet on most sentences they land in the same verdict space.
Cases like that are decided on the finished rows, not in advance.

**The independence test.** Because every lens read the source in its original
state, redundancy is measurable afterwards instead of guessed beforehand: cover
row A and ask whether row B could still have been produced from the source
alone. If B follows from A without a second look at the text, B is a duplicate
— collapse it and record that it was collapsed. A chained arrangement cannot
run this test at all: there B describes a different, rewritten text and
therefore always looks new. Statelessness is not only the protection against
distortion, it is the instrument that measures redundancy.

A **negative finding counts as a finding.** Where one lens comes back empty and
the other does not, neither follows from the other, whatever the two share on
paper.

### Polarity

A lens is *destructive* (it brings a component of the phrasing down),
*constructive* (it supplies a formulation that holds), or either depending on
the sentence. `Back-translation` is the purest falsifier — it establishes
nothing. `Form-switch` and `Node collapse` establish and never refute.
`Word ban`, `Toki Pona` and `Basque (Euskara)` go both ways.

> A combination of purely destructive lenses leaves a dismantled sentence with
> no replacement. Every usable combination contains at least one constructive
> lens, and it comes last.

### The four roles

Roles are filled in order. A combination may leave (1) empty; it may not leave
(4) empty.

| Role | Does | Typical occupants |
|---|---|---|
| (1) Opener, optional | tears the phrasing loose before any diagnosis; produces material, never evidence | `Lipogram`, `S+7 (N+7)`, `Sestina`, `Haiku`, the metres |
| (2) Reduction | states what is left when the nouns go | `Node collapse` as the standard, else `Toki Pona` |
| (3) Ablation and axis test | states what falls away; chosen by the grammatical marker of the source sentence | see the routing table |
| (4) Reconstruction | supplies the replacement phrasing | `Form-switch`, `Tagalog` (focus choice), `Walbiri (Warlpiri)` (case instead of rank), `Perspective swap`, `Latin (oratio obliqua)` (report scope marked), `Swahili (noun classes)` (class shift), `Guaraní` (epoch on the noun), `Pohnpeian (possessive classifiers)` (purpose declared), `Dialectical Midrash` (readings side by side), `Lojban` (every argument place filled) |

Purely metrical, acoustic and typographic patterns carry no theme and are
generative, not diagnostic. They belong in (1) — at most one per run, before
the diagnosis, and never cited as evidence.

### The finding matrix

The output of a combination is one row per lens, not a merged reading. The rows
are independent by construction, because every lens saw the same input.

| Lens | Forces the choice | Attaches to | Role | Polarity | Finding |
|---|---|---|---|---|---|
| `Node collapse` | thing or relation | the nouns | reduction | constructive | what survives without them |
| `Basque (Euskara)` | deliberate agent or not | the subject case | ablation | destructive | no payable ergative candidate |
| `Tuyuca` | source of the information | the verb ending | ablation | destructive | inference, given as observation |
| `Form-switch` | — | the genre | reconstruction | constructive | the replacement phrasing |

The two middle columns are the catalogue's `forced_choice` and `attachment`,
copied in unchanged. That is what makes the matrix self-checking: a duplicated
pair of cells is a collision, visible without re-reading anything.

Keep the rows; do not resolve them into a verdict. N lenses by M findings is
the result. A single semantic reading of the whole matrix is exactly the knot
the separation was built to avoid.

### Routing: from the grammatical marker to the combination

Selection follows the **grammar of the source sentence**, not its topic.

| Marker in the source sentence | Combination |
|---|---|
| any sentence, as the base | **K1** base cycle |
| collective or non-actor as subject | **K2** agent pincer |
| assertion without a stated source | **K3** source pincer |
| definite singular article for a bundle, mass or population | **K4** boundary pincer, plus a reducer |
| perfect or past tense over a running process | **K5** aspect pincer |
| genitive, possessive, reflexive | **K6** possession pincer |
| *because*, *after*, *since* in a causal function | **K7** simultaneity pincer |
| a two-word noun compound in an argument slot | **K8** compound pincer *(derived, English)* |
| vertical or motion metaphor for a number | `Guugu Yimithirr` + `Tzeltal (uphill-downhill axis)` + `Inuktitut` |

The last row is deliberately not a built-out combination: `Space & orientation`
is the smallest axis in the catalogue.

### The combinations

**K1 — base cycle.** For every sentence, whatever its content.
`Node collapse` then `Word ban` then `Back-translation` then `Form-switch`.
Reduction, ablation, counter-check, rebuild — four roles without overlap. The
ban attacks the word the reduction left standing; the back-translation is the
independent counter-check on that same word. *Stop criterion:* if the
back-translation drops nothing, the ban's suspicion is probably wrong and a
contrasting language has to take over — a metaphor shared by both languages
survives the detour, as possession does between German and English.

**K2 — agent pincer.** A subject acts that cannot act: institution,
collective, natural process, patient.
`Basque (Euskara)` then `Nez Percé (Nimipuutímt)` then
`Turkish (stacked causatives)` then `Tagalog`.
Is the ergative payable at all; is "existing" being equated with "acting
upon"; how many instigators stand between speaker and execution — the count is
always at least one and the sentence always claims none; then focus choice
supplies the replacement. A *negative* result is informative: no ergative
candidate means the sentence has a victim and no causer.

**K3 — source pincer.** The sentence gives inference, report or memory as
observation.
`Tuyuca` then `Wintu` then `Nganasan` then `Latin (oratio obliqua)`.
Five sources fanned out, then felt against read off, then the category German
lacks — *conscious myth*, for formulae everyone involved holds to be
insufficient and uses anyway. The *oratio obliqua* finally marks the whole
passage as report.

**K4 — boundary pincer.** A definite singular article disguises a population, a
mass or a bundle.
`Yucatec Maya` then `Polish (numeral threshold)` then
`Hungarian (associative plural)` then
`Mojeño Trinitario (possessive classes)`, plus `Toki Pona`.
Unbounded material without a unit; where a quantity of individuals tips into a
mass; whether a bundle is meant; whether the term is nameable without an owner.
All four are purely destructive — without the appended constructive lens the
pincer yields nothing but the news that the article is wrong.

**K5 — aspect pincer.** A perfect or past tense contracts a running process to
a point. Four completion questions that do not overlap, and a closer.
`Russian` then `Maya-Zutuhil` then `Finnish (partitive object)` then `Hopi`,
plus `Guaraní`.

| Lens | Forces the choice | Attaches to |
|---|---|---|
| `Russian` | whether the act is goal-directed or processual | `verb` |
| `Maya-Zutuhil` | how far the state has been carried | `verb` |
| `Finnish (partitive object)` | how far the event reached the object | `object` |
| `Hopi` | whether the content is manifested or still manifesting | `verb` |
| `Guaraní` | at which stage of its existence the thing is named | `noun` |

Three of the five sit at `verb` with three different forced choices — the case
the screen exists to allow. `Finnish (partitive object)` carries the sharpest
move in the chain: under negation every object turns partitive, so a denial
cannot leave its object whole, and a claim that a search came up empty cannot be
stated as a claim about a completed search.

`Guaraní` is the reconstruction, and it was missing: the other four are all
diagnostic, and a combination may not leave role (4) empty. Nominal aspect puts
the epoch on the noun (*-kue* for the former, *-rã* for the prospective), so a
measurement and its date become one word and cannot drift apart.

**K6 — possession pincer.** Genitive, possessive pronoun or reflexive.
`Navajo (inalienable possession)` then `Hawaiian (a/o possession)` then
`Mongolian (reflexive possession)` then `Pohnpeian (possessive classifiers)`.
Alienable against inalienable; the direction of dependence, acquired against
descended; whether the actor stays the anchor; what the thing is for. Four
different questions to one possessive.

**K7 — simultaneity pincer.** A *because* claims single causation — the German
reflex as such.
`Fugue` then `Latin (ablative absolute)` then `Classical Chinese (Wényán)` then
`Dialectical Midrash`.
The fugue forces the secondary voices into simultaneity instead of sequence.
The ablative absolute hangs the circumstance alongside without deciding whether
it is ground, time, condition or concession — precisely the decision *because*
makes silently. Wényán rebuilds paratactically, without causal morphology. The
Midrash lets the contradictions stand side by side.

**K8 — compound pincer.** A two-word noun compound occupies an argument slot
where a clause with named participants belongs.
`Vigraha (compound resolution)` then
`Ancient Greek (article & substantivization)` then
`Polish (instrumental predication)` then `Lojban`.
Which case relation the compound hides; whether a property is being turned into
an entity that can then persist and be defended; whether the predicate assigns a
category or states a property; then a predicate-logic rendering rebuilds it by
requiring every argument place to be filled or explicitly left open.

*Provenance:* derived from one English text, not from the German run that
produced K1 to K7. Treat it as the weakest of the eight until it has been used
more than once.

**Composition of two combinations.** A role is filled once **where its
occupants would produce the same output** — not once per role name. The
difference decides how many combinations may run on one text:

- **Reduction is global.** One invariant per text, and two reducers produce the
  same one. So `Toki Pona` drops out of K4 the moment K1 has run
  `Node collapse` — and the slot it vacates has to be filled by something
  that rebuilds rather than reduces, or the chain ends with four destructive
  lenses and no replacement.
- **Ablation and reconstruction are per axis.** Two reconstructions on two
  different axes rebuild two different things and do not collide. A text that
  triggers five combinations earns five replacement formulations, one per axis;
  collapsing them to one is the role-once rule misapplied.
- **The opener stays at most one per run**, before any diagnosis.

### Anti-combinations

Pairs that together say no more than one of them alone. `exclude_names` cannot
warn of these — it matches by exact name, so structural relatives pass straight
through — but `attachment` plus `forced_choice` does, and the generated list of
every such group lives in the repo's facets snapshot.

- **graded:** `Tuyuca` + `Tariana` — the source of the information, at the verb
  ending, twice; the second only stricter.
- **parallel:** `Nez Percé (Nimipuutímt)` + `Oromo (marked nominative)` — one
  string, `whether the participant is the one acting`, at one anchor. This is
  why K2 keeps Nez Percé and drops Oromo. `Basque (Euskara)` asks something
  else and stays, though it and Nez Percé remain borderline on most sentences.
- **parallel:** `Basque (Euskara)` + `Burushaski` — the same anchor again, a
  different choice: both read `whether the act was willed` at `subject`. Basque
  gates the ergative on deliberateness; Burushaski switches marking between a
  controlled act and an involuntary reflex. The mechanisms differ and the
  verdict does not, so either, never both. K2 takes Basque, which the rule
  permits without further reason; prefer Burushaski where intentionality is a
  matter of degree rather than a yes or no, since its entry is built on grading
  it. This pair comes from comparing the two fields, not from the run, and it
  is the kind that a reading of `focus` alone would miss — "System state
  instead of agency" and "Grading of involuntary vs. intentional causation" do
  not look alike.
- **role collision:** `Node collapse` + `Toki Pona` + `Pirahã` — three
  reduction procedures yielding the same invariant. One reducer is enough; the
  others cost space and produce the false impression of triple confirmation.
- `German` + `English` as a contrast pair — the catalogue notes the shared
  blind spot, and English carries only `Coexistence`. It delivers no finding of
  its own on causality or agency, only parataxis against hypotaxis.
- Any two purely destructive lenses without a constructive close. See K4.

### Re-deriving the routing for another source language

The roles, the polarity rule and the interference rules are about lenses and
transfer unchanged. The routing table does not: it maps **German** markers to
combinations. On a source in another language the triggers have to be re-derived
first, and the mapping is not one-to-one.

Worked through once, for English:

| Inherited German marker | What does the same work in English | What changes |
|---|---|---|
| definite singular article for a bundle | the **bare plural** — "groups", "races" | the trigger inverts. German marks with a determiner, English with the absence of one |
| *weil / seit / nach* in a causal function | the causal **preposition** — *because of*, *due to*, *as a result of* | the claim hides inside a noun phrase instead of opening a clause, and the word ban has to target a preposition |
| genitive, possessive, **reflexive** | *of*-phrase and possessive *'s* | English has no reflexive possessive at all; that third trigger has no counterpart. K6 still applies, its German marker does not exist |
| perfect or past over a running process | the **present perfect** | same trigger, sharper: the present perfect fuses a running state with a completed result more readily than the German Perfekt |
| collective or non-actor as subject | the same, and more of it | English nominalises freely, so more abstractions reach the subject slot without any morphology |

And one marker with no inherited row at all: the **two-word noun compound in an
argument slot** (*group differences*, *stereotype threat*, *open inquiry*).
German writes a compound as a single word, which keeps it legible as a compound;
the English pair reads as an adjective with its noun and the case relation
between the members vanishes without trace. K8 covers it.

The lesson generalises past English: inheriting the routing unexamined will miss
whichever marker the source language leaves invisible, and that marker tends to
be the dense one.

### Provenance and limit

K1 to K7 and the routing table are derived from one documented run of the whole
catalogue over ten German sentences. That makes them a heuristic, not a
measurement: the source language was German throughout, and the routing markers
are German markers. K8 rests on a single English text and is weaker still.

For a source in another language the roles, the polarity rule and the
interference rules carry over unchanged; the routing has to be re-derived. The
section above does that for English and found three triggers changed, one with
no counterpart at all, and one marker missing from the table entirely — so the
re-derivation is not a formality.

## Principles

- **Ground, don't invent.** Every lens comes from the `lat` catalogue; name the
  pattern's `name`, `focus` and `feature` when you use it.
- **Contrasts, not a winner.** Several reformulations side by side are worth more
  than one "best" one.
- **Always name the revelation.** A reformulation without the question "what
  becomes visible now?" is a mere stylistic exercise.
- **Faithfulness to the subject.** A reformulation may sound foreign, but it must
  hit something true about the matter.
- **Separable findings.** With several lenses, no lens reads another lens's
  output, and no two force the same choice at the same attachment point.
  Sharing an axis is allowed; sharing the question is not. Otherwise the matrix
  looks clean while the rows are no longer independent.

## Example (condensed)

Mirrors the eight workflow steps.

1. **Grasp.** Source sentence: *"The force accelerates the body because it acts
   upon it."*
2. **Diagnose.** Acting subject + suffering object + causal clause + present
   tense → axes `Causality`, `Agency & control`, `Rank & salience`,
   `Time & aspect`.
3. **Query.** (Optionally `mcp__lat__list_facets` to confirm the strings.) Search those
   axes.
4. **Input language.** The sentence is English, so exclude English — and only
   English — and prefer lenses that foreground what it backgrounds:
   - `mcp__lat__search_patterns { "kind": "language", "theme": "Time & aspect", "exclude_names": ["English"] }` → Russian, …
   - `mcp__lat__search_patterns { "kind": "language", "theme": "Perspective & reciprocity", "exclude_names": ["English"] }` → Japanese, …
   - `mcp__lat__search_patterns { "kind": "form", "theme": "Coexistence" }` → Haiku (forms are language-neutral — no `exclude_names`).
5. **Choose techniques** matching the hits (aspect view, perspective swap, form-switch).
6. **Reformulate.**
   - **Russian / aspect:** "Force — accelerating the body (ongoing)." → surfaces
     the *manner of unfolding*; the "why" recedes.
   - **Perspective swap / Japanese *wa*:** "As for the body — acceleration (is
     present)." → dissolves the *agent-patient ranking*.
   - **Form-switch / Haiku:** "A push out of the dark — / the stone rolls / on
     farther than before." → *suspends the causal chain*; the connection becomes
     an observation rather than an assertion.
7. **Name the revelation.** In mechanics there is no force without a
   counter-force — the reciprocal/simultaneous reading is often closer to the
   physics than the "because".
8. **Frame.** No version is more correct; only the access has opened up.
