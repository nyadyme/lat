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
   `search_patterns` on that `theme`. Search forms by `theme` as well — most
   carry one. Only patterns whose mechanic is purely acoustic, metrical or
   typographic carry none; reach those via `tag`/`category`/`text`. `consumes`
   and `produces` are filters too — see *Combining lenses* below.
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

## Combining lenses

Two lenses can be stacked, and the order is not free. Test any pair with one
question: **does A decide whether B has an input at all?** It runs two ways —
*withholding* (A takes B's input away: granted no bounded unit, the Finnish
partitive is forced) and *supplying* (B has no input until A makes one: the
Latin ablative absolute strips the relation the Greek particles then have to
name). Both are serial, both order-strict.

Every entry carries the stack types it `consumes` and `produces`, and both are
filters on `search_patterns`, so pairing is a lookup: read the first lens's
`produces` (Basque settles an `agent`), then ask
`search_patterns { "consumes": "agent" }` → Turkish (stacked causatives), which
turns it into an `agent-chain`. Those are its serial successors.

The other relations come off the same two fields. A producing the `-set` of what
B consumes is *distributive* (Hausa's `event-set` against Russian's `event`) —
more instances of one question, not a new one. Same input, different output is
*orthogonal* — additive, order-free. Same input **and** same output is
*redundant* — take one (Tuyuca, Tariana and Wintu all turn a `claim` into a
`claim-source`); one matching side alone is never redundancy. The match proposes
candidates, it does not decide.

A few entries only stack and are no lens alone — find them with
`search_patterns { "tag": "stacked-only" }`, never by theme. `Clinamen` is the
marked case: it takes the constraint of whichever lens is in use and breaks it
once where compliance was still available. Say which lens is being tested, and
break the rule at two sites — one where you expect it to cost nothing, one where
you expect it to cost everything; the difference is the reading.

## Background

Essay: *Language as a Tool of Thought* (CC BY 4.0) —
https://doi.org/10.5281/zenodo.21382455
