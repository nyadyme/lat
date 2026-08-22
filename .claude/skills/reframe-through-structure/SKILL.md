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
  sound so inevitable/causal".
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
   `tag`/`category`/`text`.
4. **Account for the input language (contrast, not match).** See the section
   below: exclude the user's own language and prefer language lenses that
   foreground what it backgrounds. Forms are language-neutral and always apply.
5. **Choose a technique** (see below) matching the pattern.
6. **Reformulate.** Force the subject through the structure — one, better
   several, contrasts, so that mobility arises.
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

## The four techniques

| Technique | What it does | Fits patterns whose focus is … |
|---|---|---|
| **Back-translation** | Translate a passage into a language and back; what breaks was language-bound. | robustness check; any language |
| **Form-switch** | Force the content into a foreign genre (a proof as a recipe, a process as a map). | forms: Minnesang, Haiku, Enumeration |
| **Word ban** | Drop a word for a paragraph (*because*, *leads to*, any directional verb). | causality, hierarchy |
| **Perspective swap** | Recount from the viewpoint of the supposed object (Japanese *wa*). | perspective, agent/object rank |

Language patterns supply the *grammatical lens*, form patterns the *genre lens*,
techniques the *handle*. Combine them: e.g. a perspective swap with the Japanese
topic structure, or a ban on "because" with Enumeration.

## Combining lenses

Two lenses can be stacked, and the order is not free. A pair is worth stacking
when one lens decides a precondition the other needs for its obligatory choice
— then the first can take the second's choice away, and that removal is itself
the finding. Test any pair with one question: **can A make B's question
unanswerable?**

| Relation | How the two stand to each other | Worth it |
|---|---|---|
| Serial | A decides a precondition of B and can remove B's choice | yes — the removal is the result |
| Distributive | A yields a set, B is applied to each element | sometimes — more questions, not a new kind |
| Orthogonal | Two independent sub-questions on the same object | sometimes — additive, order-free |
| Redundant | Both answer the same sub-question | no — take one |

A shared theme is neither an objection nor evidence of redundancy: serial pairs
are usually found *within* one theme, because that is where one lens's output is
the other's input. Check the sub-question, not the theme.

The fastest way to check it is a **type match, not a theme match**: does what A
delivers happen to be what B consumes? A settles whether a bounded unit exists
and B needs a unit to measure the reach of an action against; A sorts claims by
source and B needs foreign material to scope. Where A's output is B's input, the
pair is serial — and A withholding that input is the finding. Where both consume
the *same* input, the pair is redundant however far apart their themes sit. Read
both `feature` fields as input → output before deciding the relation.

Three illustrations, not a registry — derive the pair from the `feature` fields
of the two entries:

- **Node collapse → Yucatec Maya → Finnish (partitive object).** Dissolve the
  nouns into their relations, ask the residue what its unit is, then ask how far
  the action reached it. If the first step leaves no bounded thing, the partitive
  is forced and the completed reading is unavailable: whatever cannot take a
  total object was never individuated.
- **Latin (ablative absolute) → Ancient Greek (particles).** Exact inverses: the
  first withholds the relation, the second forces it to be named. Strip the
  "because", then choose the link deliberately; the distance between the two is
  the finding.
- **Tuyuca / Tariana / Wintu.** Redundant — all three ask for the source of the
  information. Stacking them adds nothing.

### Patterns that only stack

A few catalogue entries are no lens on their own — they take another lens as
their input. Find them by `tag`, never by `theme`:
`mcp__lat__search_patterns { "tag": "stacked-only" }`. `Clinamen` is the marked
case: it needs a constraint from elsewhere, holds it strictly, breaks it exactly
once at a point where compliance was still available, and reads off what the
exception cost — a rule whose breach changes nothing was never holding anything
up. So it tests the lens under it rather than the subject.

Such an entry is a layer *over* a stack, not a partner in one: put it on a
serial pair and the result is three layers, which the two-lens framing above
does not describe. That works, but say which lens is being tested and which is
doing the testing, and break the rule at two sites — one where you expect the
breach to cost nothing and one where you expect it to cost everything. The
difference between the two is the reading.

## Principles

- **Ground, don't invent.** Every lens comes from the `lat` catalogue; name the
  pattern's `name`, `focus` and `feature` when you use it.
- **Contrasts, not a winner.** Several reformulations side by side are worth more
  than one "best" one.
- **Always name the revelation.** A reformulation without the question "what
  becomes visible now?" is a mere stylistic exercise.
- **Faithfulness to the subject.** A reformulation may sound foreign, but it must
  hit something true about the matter.

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
