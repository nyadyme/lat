#!/usr/bin/env python
# SPDX-License-Identifier: Apache-2.0
# Copyright 2026 Dana Schlifka
"""Generate src/routing_seed.sql from additional_docs/lat_routing.md.

The routing markdown is the single source of truth for the combinations and
their per-language profiles. Run this after editing it:

    python tools/gen_routing.py

Pass --check to compare the generated SQL against the file on disk and exit 1
if it has fallen behind, without writing anything.

Everything that can go wrong quietly is checked against the catalogue before a
line is written. A profile is only useful if it is complete and if it never
hands a text its own grammar back as a contrast; both failures look like a
working routing in the output, so both abort here instead.
"""
import json
import sys
from pathlib import Path

from gen_seed import is_separator, parse_cells, split_list, sql_str

REPO = Path(__file__).resolve().parent.parent
CATALOG = REPO / "additional_docs" / "lat_catalog.md"
ROUTING = REPO / "additional_docs" / "lat_routing.md"
OUT = REPO / "src" / "routing_seed.sql"

SECTIONS = {
    "## Combinations": ("combinations",
                        ["id", "name", "axis", "trigger", "provenance"]),
    "## Rosters": ("rosters",
                   ["combination", "position", "lens", "kind", "role",
                    "polarity"]),
    "## Profiles": ("profiles", ["source", "own", "note"]),
    "## Routing": ("routing",
                   ["source", "combination", "verdict", "marker", "note",
                    "citation", "provenance"]),
    "## Substitutions": ("substitutions",
                         ["source", "combination", "removed", "replacement",
                          "reason"]),
}

ROLES = {"opener", "reduction", "ablation", "counter-check", "reconstruction"}
POLARITIES = {"constructive", "destructive", "both"}
KINDS = {"language", "form"}
# `fires`: the language leaves the axis implicit, the combination applies as
# written. `inverted`: the language already forces the choice, and the
# combination tests whether the forced choice was warranted. There is no
# "void": a combination whose axis the language makes trivial still has a
# marker, and saying so is the profile's job, not leaving the row out.
VERDICTS = {"fires", "inverted"}
PROVENANCES = {"run", "derived", "checked"}


def fail(message):
    raise SystemExit(f"{ROUTING.name}: {message}")


def load_catalogue():
    """Map every catalogue name to (kind, forced_choice, attachment, status)."""
    out = {}
    kind = None
    for line in CATALOG.read_text(encoding="utf-8").splitlines():
        s = line.strip()
        if s.startswith("## Languages"):
            kind = "language"
            continue
        if s.startswith("## Forms"):
            kind = "form"
            continue
        if s.startswith("##"):
            kind = None
            continue
        if kind is None or not s.startswith("|"):
            continue
        cells = parse_cells(line)
        if cells[0] == "Name" or is_separator(cells):
            continue
        # name, category, classification, focus, feature, forced_choice,
        # attachment, description, tags, themes, source, status
        out[cells[0]] = (kind, cells[5], cells[6], cells[11])
    if not out:
        raise SystemExit(f"{CATALOG.name}: no entries parsed")
    return out


def load_routing():
    """Read the five tables of the routing file into lists of dicts."""
    tables = {key: [] for key, _ in SECTIONS.values()}
    current = None
    for line in ROUTING.read_text(encoding="utf-8").splitlines():
        s = line.strip()
        # Only a level-2 heading opens or closes a table section. A deeper
        # heading inside one is a sub-heading, and treating it as a section
        # change used to drop every row after it without a word.
        if s.startswith("## "):
            current = SECTIONS.get(s)
            continue
        if s.startswith("#"):
            continue
        if current is None or not s.startswith("|"):
            continue
        key, cols = current
        cells = parse_cells(line)
        if is_separator(cells) or cells[0] in {"Id", "Combination",
                                                "Source language"}:
            continue
        if len(cells) != len(cols):
            fail(f"{key} row {cells[0]!r} has {len(cells)} cells, "
                 f"expected {len(cols)} ({', '.join(cols)})")
        tables[key].append(dict(zip(cols, cells)))
    for key, rows in tables.items():
        if not rows:
            fail(f"no rows parsed in the {key} table")
    return tables


def check_combinations(tables):
    ids = [c["id"] for c in tables["combinations"]]
    if len(ids) != len(set(ids)):
        fail(f"duplicate combination id in {ids}")
    for c in tables["combinations"]:
        if not c["trigger"] or not c["provenance"]:
            fail(f"{c['id']}: trigger and provenance may not be empty")
    return set(ids)


def check_rosters(tables, ids, catalogue):
    rosters = {cid: [] for cid in ids}
    for r in tables["rosters"]:
        cid, lens = r["combination"], r["lens"]
        if cid not in ids:
            fail(f"roster row {lens!r} names unknown combination {cid!r}")
        if r["kind"] not in KINDS:
            fail(f"{cid} {lens!r}: unknown kind {r['kind']!r}")
        if r["role"] not in ROLES:
            fail(f"{cid} {lens!r}: unknown role {r['role']!r}")
        if r["polarity"] not in POLARITIES:
            fail(f"{cid} {lens!r}: unknown polarity {r['polarity']!r}")
        if not r["position"].isdigit():
            fail(f"{cid} {lens!r}: position {r['position']!r} is no number")
        check_lens(lens, f"{cid} roster", catalogue, r["kind"])
        rosters[cid].append(r)
    for cid, rows in rosters.items():
        positions = [int(r["position"]) for r in rows]
        if sorted(positions) != list(range(1, len(rows) + 1)):
            fail(f"{cid}: positions {positions} are not 1..{len(rows)}")
        rows.sort(key=lambda r: int(r["position"]))
        # The doctrine's rule is about polarity, not about the role name:
        # every usable combination carries a constructive lens and it comes
        # last. K4 closes on a reducer that can build (Toki Pona), which a
        # check on the role alone would reject.
        if rows and rows[-1]["polarity"] == "destructive":
            fail(f"{cid}: the last lens {rows[-1]['lens']!r} is purely "
                 "destructive; a combination must close on a lens that can "
                 "build")
    return rosters


def check_lens(name, where, catalogue, kind=None):
    if name not in catalogue:
        fail(f"{where}: {name!r} is not in the catalogue")
    have_kind, _, _, status = catalogue[name]
    if kind is not None and have_kind != kind:
        fail(f"{where}: {name!r} is a {have_kind}, the row says {kind}")
    if status != "sourced":
        fail(f"{where}: {name!r} is {status or 'unsourced'} and may not ride "
             "in a combination")


def check_profiles(tables, catalogue):
    own = {}
    for p in tables["profiles"]:
        # The database keys profiles case-insensitively, so two names that
        # differ only in case pass here and fail the seed at server start.
        twin = [name for name in own if name.lower() == p["source"].lower()]
        if twin:
            fail(f"profile {p['source']!r} is listed twice (as {twin[0]!r}); "
                 "profile names are matched without regard to case")
        names = split_list(p["own"])
        for name in names:
            if name not in catalogue:
                fail(f"profile {p['source']}: own entry {name!r} is not in "
                     "the catalogue")
        own[p["source"]] = names
    return own


def check_routing(tables, ids, own):
    seen = {}
    for r in tables["routing"]:
        src, cid = r["source"], r["combination"]
        where = f"routing {src} {cid}"
        if src not in own:
            fail(f"{where}: no profile for {src!r}")
        if cid not in ids:
            fail(f"{where}: unknown combination")
        if (src, cid) in seen:
            fail(f"{where}: listed twice")
        if r["verdict"] not in VERDICTS:
            fail(f"{where}: unknown verdict {r['verdict']!r}; "
                 f"allowed: {sorted(VERDICTS)}")
        if r["provenance"] not in PROVENANCES:
            fail(f"{where}: unknown provenance {r['provenance']!r}")
        if not r["marker"]:
            fail(f"{where}: no marker")
        if not r["citation"]:
            # Same rule as the catalogue's source column: a marker nobody
            # checked against a grammar is a marker written from recall.
            fail(f"{where}: no citation")
        seen[(src, cid)] = r
    for src in own:
        missing = sorted(ids - {cid for s, cid in seen if s == src})
        if missing:
            fail(f"profile {src!r} gives no verdict for {missing}; a missing "
                 "row would read as 'does not apply' when nobody looked")


def effective_roster(roster, subs):
    """Apply a profile's substitutions to one roster; returns (lenses, dropped)."""
    by_removed = {s["removed"]: s for s in subs}
    lenses, dropped = [], []
    for r in roster:
        sub = by_removed.get(r["lens"])
        if sub is None:
            lenses.append(r["lens"])
        elif sub["replacement"]:
            lenses.append(sub["replacement"])
        else:
            dropped.append(r["lens"])
    return lenses, dropped


def check_substitutions(tables, rosters, own, catalogue):
    grouped = {}
    for s in tables["substitutions"]:
        src, cid, removed = s["source"], s["combination"], s["removed"]
        where = f"substitution {src} {cid} {removed!r}"
        if src not in own:
            fail(f"{where}: no profile for {src!r}")
        if cid not in rosters:
            fail(f"{where}: unknown combination")
        if removed not in [r["lens"] for r in rosters[cid]]:
            fail(f"{where}: {removed!r} is not in the {cid} roster")
        if not s["reason"]:
            fail(f"{where}: no reason")
        replacement = s["replacement"]
        if replacement:
            check_lens(replacement, where, catalogue)
            if replacement in own[src]:
                fail(f"{where}: the replacement is an own entry of {src}")
        key = (src, cid)
        if removed in [x["removed"] for x in grouped.get(key, [])]:
            fail(f"{where}: listed twice")
        grouped.setdefault(key, []).append(s)

    for src, names in own.items():
        for cid, roster in rosters.items():
            subs = grouped.get((src, cid), [])
            lenses, _ = effective_roster(roster, subs)
            leaked = sorted(set(lenses) & set(names))
            if leaked:
                fail(f"{src} {cid}: own entries {leaked} still ride in the "
                     "roster; substitute or drop them for this profile")
            if len(lenses) != len(set(lenses)):
                fail(f"{src} {cid}: a replacement is already in the roster "
                     f"({lenses})")
            keys = {}
            for lens in lenses:
                _, choice, anchor, _ = catalogue[lens]
                if not choice or not anchor:
                    # An empty cell is not a shared question; comparing it
                    # would report two unfilled entries as a collision.
                    continue
                other = keys.get((choice, anchor))
                if other:
                    fail(f"{src} {cid}: {other!r} and {lens!r} force the same "
                         f"choice ({choice!r}) at the same anchor "
                         f"({anchor!r})")
                keys[(choice, anchor)] = lens
    return grouped


def render(tables, catalogue):
    parts = [
        "-- SPDX-License-Identifier: Apache-2.0",
        "-- Copyright 2026 Dana Schlifka",
        "--",
        "-- GENERATED from additional_docs/lat_routing.md by "
        "tools/gen_routing.py.",
        "-- Do not edit by hand; edit the routing file and regenerate.",
        "-- Applied on start when the routing tables are empty.",
        "-- own_entries is a JSON array; an empty replacement means dropped.",
        "",
    ]

    def insert(table, cols, rows):
        values = ",\n".join(
            "    (" + ", ".join(row) + ")" for row in rows)
        parts.append(f"INSERT INTO {table}\n    ({', '.join(cols)})\n"
                     f"VALUES\n{values};\n")

    insert("combinations", ["id", "name", "axis", "trigger_text", "provenance"],
           [[sql_str(c[k]) for k in ("id", "name", "axis", "trigger",
                                     "provenance")]
            for c in tables["combinations"]])
    insert("combination_lenses",
           ["combination", "position", "name", "kind", "role", "polarity"],
           [[sql_str(r["combination"]), r["position"], sql_str(r["lens"]),
             sql_str(r["kind"]), sql_str(r["role"]), sql_str(r["polarity"])]
            for r in tables["rosters"]])
    insert("routing_profiles", ["source_language", "own_entries", "note"],
           [[sql_str(p["source"]),
             sql_str(json.dumps(split_list(p["own"]), ensure_ascii=False)),
             sql_str(p["note"])]
            for p in tables["profiles"]])
    insert("routing",
           ["source_language", "combination", "verdict", "marker", "note",
            "citation", "provenance"],
           [[sql_str(r[k]) for k in ("source", "combination", "verdict",
                                     "marker", "note", "citation",
                                     "provenance")]
            for r in tables["routing"]])
    # The replacement's kind travels with the row: the server must not have to
    # look it up in a catalogue that may be older than this routing.
    insert("substitutions",
           ["source_language", "combination", "removed", "replacement",
            "replacement_kind", "reason"],
           [[sql_str(s["source"]), sql_str(s["combination"]),
             sql_str(s["removed"]), sql_str(s["replacement"]),
             sql_str(catalogue[s["replacement"]][0] if s["replacement"]
                     else ""),
             sql_str(s["reason"])]
            for s in tables["substitutions"]])
    return "\n".join(parts)


def main():
    catalogue = load_catalogue()
    tables = load_routing()
    ids = check_combinations(tables)
    rosters = check_rosters(tables, ids, catalogue)
    own = check_profiles(tables, catalogue)
    check_routing(tables, ids, own)
    check_substitutions(tables, rosters, own, catalogue)

    text = render(tables, catalogue)
    if "--check" in sys.argv[1:]:
        current = OUT.read_text(encoding="utf-8") if OUT.exists() else ""
        if current != text:
            print(f"{OUT.relative_to(REPO)} is stale; run "
                  "python tools/gen_routing.py", file=sys.stderr)
            return 1
        print(f"{OUT.relative_to(REPO)} is current")
        return 0

    # newline="\n" for the same reason as gen_seed.py: the platform default
    # rewrites every line on Windows.
    OUT.write_text(text, encoding="utf-8", newline="\n")
    print(f"wrote {OUT.relative_to(REPO)}: {len(ids)} combinations, "
          f"{len(own)} profiles, {len(tables['routing'])} routing rows, "
          f"{len(tables['substitutions'])} substitutions")
    return 0


if __name__ == "__main__":
    sys.exit(main())
