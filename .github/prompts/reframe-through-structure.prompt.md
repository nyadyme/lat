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
   map it to one cognitive axis: `Causality`, `Agency & rank`, `Time & aspect`,
   `Coexistence`, `Perspective & reciprocity`, `Object boundaries`,
   `Evidence & certainty`, `Space & orientation`, `Possession & belonging`,
   `Logic & ambiguity`.
3. **Query.** Call `list_facets` if unsure of exact values, then
   `search_patterns` on that `theme`. Purely formal patterns (meters, fixed
   forms) carry no theme — reach them via `tag`/`category`/`text`.
4. **Input language (contrast, not match).** Detect the language the user writes
   in and pass `exclude_names: ["<that language>"]` so the structure they
   already think in is not recommended back. English and German are a baseline
   pair (shared causal-chain bias) — for either, exclude **both**. Prefer
   language lenses that foreground the axis the user's language leaves implicit.
   Forms and techniques are language-neutral and always apply.
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
