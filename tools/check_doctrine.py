#!/usr/bin/env python
# SPDX-License-Identifier: Apache-2.0
# Copyright 2026 Dana Schlifka
"""Check the workflow's combinations against the catalogue.

    python tools/check_doctrine.py

The skill names patterns in prose, and prose goes stale silently. Two ways it
has already gone wrong: a renamed entry left dangling references behind, and
two pincers carried contested lenses for as long as nobody counted. Both are
invisible until someone runs a combination and gets a lens that no longer
exists, or a matrix that reads as settled while one of its rows is still being
argued over in the literature.

Two things are checked. First the rosters of the combinations — the recital
of lenses under each `**Kn — ...**` heading, the lens tables inside those
sections, and the combination column of the routing table. Every name there
must exist in the catalogue, and none may be `contested`. Second each catalogue
entry against itself: a row whose prose names an axis it does not carry is
where a correction stopped at one cell.

What is deliberately not checked: the skill's prose. A section may name a
contested lens to say that it was taken out, which is exactly what K2 and K5
now do, and a collision note may name one to warn about it. Flagging those
would push the doctrine toward not mentioning what it excludes.
"""
import re
import sys
from pathlib import Path

REPO = Path(__file__).resolve().parent.parent
CATALOG = REPO / "additional_docs" / "lat_catalog.md"
SKILL = REPO / ".claude" / "skills" / "reframe-through-structure" / "SKILL.md"

# The rule the rosters are checked against has to be in the file that states
# it; a check whose premise can be deleted silently guards nothing.
RULE_MARKER = "exclude_contested"

# Pairs the byte comparison cannot see: different strings, one anchor, and on
# most sentences one verdict space. Swept out of the 39 groups that share an
# anchor and an axis. Two of a pair in one combination make a matrix look
# like two measurements when it holds one.
ADJACENT = [
    ("Tagalog", "Wolof (focus conjugation)",
     "both settle what the verb puts in front, one by voice and one by focus"),
    ("Mise en abyme", "Scale test",
     "self-similarity asserted against self-similarity tested — inverse "
     "moves on one object"),
    ("Node collapse", "Vigraha (compound resolution)",
     "both dissolve nouns into the relations they hide"),
    ("Russian", "Biblical Hebrew",
     "perfective against imperfective, twice, at the same anchor"),
    ("Nganasan", "Mano (auxiliary series)",
     "denial against grade of assertion — opposite directions, one anchor"),
    ("Amele (switch-reference)", "Hua (Yagaria)",
     "whether the subject carries over against who the next one is — the "
     "second answers the first"),
    ("Basque (Euskara)", "Dyirbal (syntactic ergativity)",
     "one merge of actor and undergoer, inside a clause and across a chain"),
]

# A family is a set where at most one member may ride in a combination. Pairs
# would need fifteen entries to say this; the members differ only in *what*
# returns, and any two of them measure the same thing twice.
FAMILIES = [
    ("return-forms",
     {"Sestina", "Villanelle", "Triolet", "Rondo", "Litany / Anaphora",
      "Glosa"},
     "each asks what the passage is made to come back to, and they differ "
     "only in what returns — six words, two lines, a core, a variable, four "
     "given lines"),
]

# Checked by hand and found genuinely independent; listed so the next sweep
# does not re-open them. Nez Percé refuses the merge that Basque performs.
CLEARED = {frozenset({"Basque (Euskara)", "Nez Percé (Nimipuutímt)"})}

# An entry whose prose names an axis by name should also carry it. Six of its
# cells restate one claim, so correcting the claim in one leaves the others
# asserting the axis that was just taken away — the failure this catalogue has
# repeated more than any other, and the only one with a mechanical handle.
AXIS_CUE = {
    "Causality": r"\bcaus\w*|\bbecause\b|\bleads to\b|cause and effect",
    "Time & aspect": r"\barrow of time\b|\btense\b|\btime line\b|\btemporal\b",
    "Possession & belonging": r"\bpossess\w*|\bowner\b|\balienab\w*",
    "Space & orientation": r"\bspatial\b|\bcardinal direction\b",
    "Evidence & certainty": r"\bevidential\w*|\bcertainty\b",
}

# Reviewed and correct: the word is incidental, not a claim about the axis. A
# description may say a language has no tense without being a tense lens, and
# "because" is a conjunction before it is a category.
AXIS_REVIEWED = {
    ("English", "Causality"),
    ("Basque (Euskara)", "Causality"),
    ("Georgian (Kartuli)", "Causality"),
    ("Sanskrit (nominal compound)", "Causality"),
    ("Classical Chinese (Wényán)", "Time & aspect"),
    ("Wolof (focus conjugation)", "Time & aspect"),
    ("Amharic (converb chains)", "Time & aspect"),
    ("Polish (masculine-personal plural)", "Time & aspect"),
    ("Jingulu (three light verbs)", "Time & aspect"),
    ("Yoruba (serial verbs)", "Time & aspect"),
    ("Tz'utujil", "Possession & belonging"),
    ("Nganasan", "Possession & belonging"),
    ("Vigraha (compound resolution)", "Possession & belonging"),
}

NAME = re.compile(r"`([^`]+)`")
HEADING = re.compile(r"^\*\*(K\d)\b")
# The line that recites a combination: nothing but backticked names and the
# words that join them. Prose about a combination never passes this.
JOINERS = {"then", "plus", "and", "", ".", ",", "+", ";"}


def catalogue():
    """Map every pattern name to its status."""
    out = {}
    kind = None
    for line in CATALOG.read_text(encoding="utf-8").splitlines():
        s = line.strip()
        if s.startswith("## Languages"):
            kind = True
            continue
        if s.startswith("## Forms"):
            kind = True
            continue
        if s.startswith("##"):
            kind = None
            continue
        if not kind or not s.startswith("|"):
            continue
        cells = [c.strip() for c in s.strip("|").split("|")]
        if len(cells) != 12 or cells[0] == "Name":
            continue
        if all(c and set(c) <= set("-: ") for c in cells):
            continue
        out[cells[0]] = cells
    if not out:
        raise SystemExit(f"{CATALOG.name}: no entries parsed")
    return out


def is_roster(line):
    """True when a line holds only lens names and the words joining them."""
    if "`" not in line:
        return False
    rest = NAME.sub(" ", line)
    return all(tok.strip(".,;") in JOINERS for tok in rest.split())


def references(text):
    """Yield (where, name) for every lens a combination actually enlists."""
    lines = text.splitlines()
    section = None
    in_routing = False
    for no, line in enumerate(lines, 1):
        s = line.strip()

        if HEADING.match(s):
            section = HEADING.match(s).group(1)
        elif s.startswith("### ") or s.startswith("## "):
            section = None

        # The routing table maps a marker in the source to a combination; its
        # last row spells one out instead of naming it.
        if s.startswith("| Marker in the source sentence"):
            in_routing = True
            continue
        if in_routing and not s.startswith("|"):
            in_routing = False

        if in_routing and s.startswith("|"):
            for name in NAME.findall(s):
                yield (f"routing table, line {no}", name)
            continue

        if section is None:
            continue
        if is_roster(s):
            for name in NAME.findall(s):
                yield (f"{section} roster, line {no}", name)
        elif s.startswith("| `"):
            first = NAME.search(s)
            if first:
                yield (f"{section} lens table, line {no}", first.group(1))


def main():
    status = catalogue()
    text = SKILL.read_text(encoding="utf-8")

    problems = []
    if RULE_MARKER not in text:
        problems.append(
            f"{SKILL.name}: the rule these rosters are checked against is "
            f"gone — no mention of {RULE_MARKER!r} is left in the doctrine")

    enlisted = {}
    seen = 0
    for where, name in references(text):
        seen += 1
        enlisted.setdefault(where.split(" ")[0], set()).add(name)
        if name not in status:
            problems.append(
                f"{where}: {name!r} is not in the catalogue — renamed or "
                "removed, and the combination still calls for it")
        elif status[name][11] == "contested":
            problems.append(
                f"{where}: {name!r} is contested and may not ride in a "
                "combination; use it as a single lens in pass 1, marked")

    for a, b, why in ADJACENT:
        if frozenset({a, b}) in CLEARED:
            continue
        for combination, names in enlisted.items():
            if a in names and b in names:
                problems.append(
                    f"{combination}: {a!r} and {b!r} ride together, but they "
                    f"are adjacent — {why}. Different strings hide it from "
                    "the byte comparison; take one of the two.")

    for label, members, why in FAMILIES:
        for combination, names in enlisted.items():
            together = sorted(members & names)
            if len(together) > 1:
                problems.append(
                    f"{combination}: {together} are all {label}, and a "
                    f"combination takes at most one — {why}.")

    used = set()
    for name, cells in sorted(status.items()):
        carried = {x.strip() for x in cells[9].split(",") if x.strip()}
        prose = " ".join(cells[i] for i in (2, 3, 4, 5, 7, 8)).lower()
        for axis, pattern in AXIS_CUE.items():
            found = re.search(pattern, prose)
            if axis in carried or not found:
                continue
            if (name, axis) in AXIS_REVIEWED:
                used.add((name, axis))
                continue
            problems.append(
                f"{name}: the prose says {found.group(0)!r} but the entry "
                f"does not carry {axis!r} — either a cell kept a claim "
                "the axis lost, or the axis belongs back")

    # A clearance outlives what it cleared: the entry gets rewritten, the cue
    # stops firing, and the line stays on as a standing permission nobody
    # reads. Then the list is no longer the record of a review.
    for name, axis in sorted(AXIS_REVIEWED - used):
        problems.append(
            f"AXIS_REVIEWED holds ({name!r}, {axis!r}), but nothing fires "
            "there any more — the entry was renamed or rewritten under the "
            "clearance; drop the line")

    for problem in problems:
        print(problem, file=sys.stderr)
    if problems:
        print(f"\n{len(problems)} problem(s) in {seen} enlisted lenses "
              f"and {len(status)} entries", file=sys.stderr)
        return 1

    print(f"doctrine ok: {seen} enlisted lenses, all present and sourced; "
          f"{len(status)} entries agree with their own axes")
    return 0


if __name__ == "__main__":
    sys.exit(main())
