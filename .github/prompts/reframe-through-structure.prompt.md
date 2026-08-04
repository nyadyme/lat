---
mode: agent
description: Reframe a text through a contrasting language or bound form (via the lat MCP) to surface hidden structural assumptions.
---

# Reframe through structure

Use the **lat** MCP server to reframe a problem or text through a contrasting
language or poetic form, so the structure the original phrasing hid becomes
visible. Tools: `search_patterns`, `get_pattern`, `list_patterns`,
`list_facets`. If lat is not connected, say so and stop (see the repository
README for setup).

Ground every lens in the lat catalogue — cite the pattern's `name`, `focus`, and
`feature`. **Never invent** languages or forms.

## Workflow

1. **Grasp.** Get the concrete sentence/paragraph. If none is given (or nothing
   is selected in the editor), ask for one.
2. **Diagnose the bias.** Name the quiet structure of the source phrasing —
   causal chain? agent/object rank? fixed tense? tacit object boundaries? — and
   map it to one cognitive axis: `Causality`, `Agency & control`,
   `Rank & salience`, `Time & aspect`, `Coexistence`,
   `Perspective & reciprocity`, `Object boundaries`, `Evidence & certainty`,
   `Space & orientation`, `Possession & belonging`, `Logic & ambiguity`.
   `Agency & control` is the actor's relation to the act; `Rank & salience`
   which participant stands in front of the others.
3. **Query.** Call `list_facets` if unsure of exact values, then
   `search_patterns` on that `theme`. Purely formal patterns (meters, fixed
   forms) carry no theme — reach them via `tag`/`category`/`text`.
4. **Input language (contrast, not match).** Detect the language of the *text
   being analysed* — not the language of the request — and pass
   `exclude_names: ["<that language>"]` so the structure the text already thinks
   in is not recommended back. Exclude exactly that **one** language: never a
   pair, a family, or a list of relatives. A closely related language stays a
   valid lens, because kinship is never uniform across the axes — English and
   German share the causal-chain reflex, yet English parataxis carries
   *coexistence* where German hypotaxis subordinates. Judge per axis, not per
   family. Prefer lenses that foreground the axis the input language leaves
   implicit. Forms and techniques are language-neutral and always apply.
5. **Choose a technique** matching the pattern: back-translation, form-switch,
   word ban, or perspective swap.
6. **Reformulate** the text through the chosen lens(es) — offer several
   contrasts, not one "winner".
7. **Name the revelation.** For each reformulation, state explicitly *what* it
   makes visible that the source phrasing concealed. This is the payoff, not the
   wording.
8. **Frame.** No version is "more correct"; the goal is mobility — showing the
   difficulty sat in the form of telling, not in the problem.

## Background

Essay: *Language as a Tool of Thought* (CC BY 4.0) —
https://doi.org/10.5281/zenodo.21382455
