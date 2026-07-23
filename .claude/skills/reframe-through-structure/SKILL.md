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

- `list_facets` — which themes / categories / classifications / tags exist.
- `search_patterns` — find patterns by filters (`kind`, `theme`, `category`,
  `classification`, `focus`, `tag`, `text`, `exclude_names`; all optional,
  combined with AND).
- `get_pattern` — full details of a pattern (`kind` + `name`).
- `list_patterns` — overview.

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
   the diagnosed bias maps to one of them: `Causality`, `Agency & rank`,
   `Time & aspect`, `Coexistence`, `Perspective & reciprocity`,
   `Object boundaries`, `Evidence & certainty`, `Space & orientation`,
   `Possession & belonging`, `Logic & ambiguity`. Call `list_facets` if unsure
   of the exact strings, then `search_patterns` on that `theme`. Pull details
   with `get_pattern` as needed (focus, feature). Purely formal tools (meters,
   fixed forms) carry no theme — reach them via `tag`/`category`/`text`.
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
biased axis compared with the user's own language. Russian aspect is a
revelation for someone writing in English or German (where aspect is not
obligatory) but banal for someone whose language already forces it. So:

- **Detect the input language** from the user's message (no tool needed — you
  read it directly).
- **Exclude it.** Pass `exclude_names: ["<that language>"]` to `search_patterns`
  so the structure the user already thinks in is not recommended back to them.
  Note `exclude_names` matches by **exact name only** — a hard guarantee for the
  named entries, nothing more. Structural relatives are *not* auto-excluded, so
  down-weight them yourself (agent judgement, not a tool operation). In
  particular English and German are the baseline pair: both carry the
  causal-chain / agent-patient bias, so for a German **or** English input exclude
  **both** (`exclude_names: ["German", "English"]`) — otherwise they recommend
  each other as "contrast" when they are the same blind spot.
- **Prefer complementarity.** The best lens foregrounds (makes obligatory) the
  very axis the user's language leaves implicit. A German/English speaker stuck
  on causality gains most from aspect (Russian), evidentiality (Tuyuca), animacy
  (Ojibwe), or absolute space (Guugu Yimithirr) — axes their language backgrounds.
- **Forms are language-neutral.** A Haiku, a word ban, a Sestina works in any
  language, so form/technique choices do not depend on the input language; only
  the *language* lenses do.

If the user's language is itself in the catalogue, look it up (its `focus` and
`themes` describe what it foregrounds) to reason about its blind spots; if not,
infer its rough profile.

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
   tense → axes `Causality`, `Agency & rank`, `Time & aspect`.
3. **Query.** (Optionally `list_facets` to confirm the strings.) Search those
   axes.
4. **Input language.** The sentence is English — a baseline causal-chain
   language — so exclude both baselines and prefer lenses that foreground what
   they background:
   - `search_patterns { "kind": "language", "theme": "Time & aspect", "exclude_names": ["English", "German"] }` → Russian, …
   - `search_patterns { "kind": "language", "theme": "Perspective & reciprocity", "exclude_names": ["English", "German"] }` → Japanese, …
   - `search_patterns { "kind": "form", "theme": "Coexistence" }` → Haiku (forms are language-neutral — no `exclude_names`).
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
