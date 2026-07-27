# Kirchhoff's laws through the toolbox — Japanese input

Same source problem as [`kirchhoff-de-input.md`](kirchhoff-de-input.md), but the
input language is **Japanese**. This shows the skill's contrast rule at work:
*the same problem yields different lenses depending on the input language.*

**Input language:** Japanese → `exclude_names: ["Japanese"]`. Crucially,
**Perspective & reciprocity is Japanese's native strength** (topic-prominence,
agent-dropping), so those language lenses are *down-weighted* — they add little a
Japanese speaker doesn't already do. The illuminating axes are the mirror image
of the German case: the ones Japanese leaves **implicit**.

**Source.** KCL: ΣI at a node = 0. KVL: ΣU in a loop = 0.

## 1 · Diagnose the bias (Japanese phrasing)

A Japanese statement is naturally topic-comment and agentless — roughly
"節点については、電流の和はゼロ" ("as for the node, the sum of currents is
zero"). So the **causal / agentive artifact German carries is already gone**.
What Japanese leaves *unmarked* instead:

- **Quantifier scope / plurality** — "the sum of currents" over *which* set? soft.
- **Evidential and modal status** — is ΣI = 0 observed, inferred, or definitional?
  grammatically unmarked.
- **Explicit logical form.**

So for a Japanese speaker the bias to break runs the **opposite direction** from
German: not toward agentlessness (they have it) but toward **explicitness** —
axes `Logic & ambiguity` and `Evidence & certainty`.

## 2 · Contrast selection

Exclude Japanese; down-weight Perspective & reciprocity language lenses (Georgian,
Tagalog, Thai, Hua — near-native). Lean on what Japanese backgrounds. Grounded
lenses (with `focus`):

- **Lojban** — *"First-order predicate logic as syntax"* (Logic & ambiguity)
- **Ithkuil** — *"Elimination of ambiguity; hyper-precise categorization"* (Logic & ambiguity)
- **Sanskrit (Syādvāda)** — *"Multidimensional relativity of truth"* (Logic & ambiguity)
- **Tuyuca** — *"Verifiable origin of information"* (Evidence & certainty)
- **Classical Chinese** — *"Pure relational constellation in the now"* (Coexistence; isolating, particle-free — a structural contrast to particle-heavy Japanese)
- **Koan** — *"Short-circuiting the logical-analytical mind"* (form, Logic & ambiguity)

## 3 · Reformulations

### Node rule (KCL)

- **Lojban / predicate logic:** "For node n: (sum over every branch b incident
  to n of signed_current(b)) = 0." Every quantifier and set made explicit; no
  topic-comment softness. → *reveals:* Japanese leaves *which* currents and *over
  what set* implicit — the law is a precise universally-quantified relation.
- **Tuyuca / evidential:** attach the source-marker — is ΣI = 0 *seen*
  (measured), *inferred* (from charge conservation), or *assumed* (idealization)?
  Mark it inferred-from-principle. → *reveals:* KCL is **not an observation** but
  an inference from charge conservation — a status neither Japanese nor German
  forces you to declare.
- **Classical Chinese / isolating parallel:** "節: 入, 出 — 齊 (level)", bare
  parallel, no particles. → *reveals:* the balance stands without any
  topic-marker scaffolding; even the *wa*-frame is optional dressing.

*Revelation (KCL):* the Japanese framing already removed the causer; the detour
instead surfaces the **unstated set and epistemic status** — over which branches,
and known how.

### Loop rule (KVL)

- **Ithkuil / modal precision:** encode whether ΣU = 0 is *necessary,
  definitional, or contingent* — forcing the statement that it is **definitional**
  (the potential is single-valued), not empirically caused. → *reveals:* the
  *kind* of truth, which topic-comment leaves open.
- **Sanskrit (Syādvāda) / sevenfold:** "in a respect ΣU = 0 (ideal); in a respect
  not (a real loop has stray EMF / inductance); in a respect indescribable (the
  idealization boundary)." → *reveals:* the **idealization** — KVL holds *in a
  respect*, acknowledging the model's scope that a flat assertion hides.
- **Koan (form):** "Around a loop that returns to itself — where did the voltage
  go?" a paradox against the additive reflex. → *reveals:* the **return-to-self**
  (path-independence) as the essence, not the summing.

*Revelation (KVL):* for a Japanese speaker the useful move is toward **logical
and modal explicitness** — what sort of truth, under what idealization — not
toward dissolving an agent that the grammar never imposed.

## 4 · Overarching revelation

The laws are unchanged; what changes is **the blind spot worth illuminating**.
German narrates a balance causally and hierarchically, so the detour runs toward
*coexistence and agentlessness*. Japanese already speaks the balance
agentlessly, so the detour runs the other way — toward *explicit quantification,
evidential status, and the kind of truth*. Same Σ = 0, mirror-image lenses. That
divergence **is** the contrast rule working as intended.

## 5 · Framing

No version is more correct. The instructive point is meta: which reframing helps
depends on the structure the speaker already inhabits. `exclude_names` guarantees
the source language itself is never recommended back; the rest — down-weighting a
speaker's native axes — is the agent's judgement.
