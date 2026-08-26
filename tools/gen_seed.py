#!/usr/bin/env python
# SPDX-License-Identifier: Apache-2.0
# Copyright 2026 Dana Schlifka
"""Generate src/seed.sql from additional_docs/lat_catalog.md.

The catalogue markdown is the single source of truth. Run this after editing it:

    python tools/gen_seed.py

Then rebuild; on a fresh (empty) database the new seed is applied on first start
(delete the database file to reseed an existing one).
"""
import json
import sys
from pathlib import Path

REPO = Path(__file__).resolve().parent.parent
CATALOG = REPO / "additional_docs" / "lat_catalog.md"
OUT = REPO / "src" / "seed.sql"

# Column order as it appears in the catalogue tables.
COLS = ["name", "category", "classification", "focus", "feature",
        "forced_choice", "attachment", "description", "tags", "themes"]

# Closed vocabulary for `attachment`: the constituent a pattern interrogates.
# Kept coarse on purpose — the finer the anchor, the more collisions between
# two lenses go unnoticed, and `forced_choice` does the fine separation.
ATTACHMENTS = {
    "verb", "subject", "object", "noun", "possessive", "person",
    "spatial frame", "connective", "word order", "whole passage", "surface",
}

# Closed cognitive-axis vocabulary for `themes`. Multi-valued, and allowed to
# be empty: a pattern whose mechanic is purely acoustic, metrical or
# typographic carries no axis and is reached via tags/category instead.
THEMES = {
    "Causality", "Agency & control", "Rank & salience", "Time & aspect",
    "Coexistence", "Perspective & reciprocity", "Object boundaries",
    "Evidence & certainty", "Space & orientation", "Possession & belonging",
    "Logic & ambiguity",
}


def parse_cells(line):
    inner = line.strip()
    if inner.startswith("|"):
        inner = inner[1:]
    if inner.endswith("|"):
        inner = inner[:-1]
    return [c.strip() for c in inner.split("|")]


def is_separator(cells):
    return all(c and set(c) <= set("-: ") for c in cells)


def sql_str(v):
    return "'" + v.replace("'", "''") + "'"


def split_list(cell):
    """Split a comma-separated cell; an empty cell yields no items."""
    return [x.strip() for x in cell.split(",") if x.strip()]


def json_arr(cell):
    return json.dumps(split_list(cell), ensure_ascii=False)


def check_vocabulary(row):
    """Abort on a value outside one of the closed vocabularies.

    Both fields are load-bearing and fail quietly if wrong: an unknown
    attachment drops a pattern out of every anchor comparison, and a mistyped
    theme makes it unreachable by the theme filter that is the main way in.
    """
    if row["attachment"] not in ATTACHMENTS:
        raise SystemExit(
            f"{row['name']}: unknown attachment {row['attachment']!r}; "
            f"allowed: {sorted(ATTACHMENTS)}")
    for theme in split_list(row["themes"]):
        if theme not in THEMES:
            raise SystemExit(
                f"{row['name']}: unknown theme {theme!r}; "
                f"allowed: {sorted(THEMES)}")


def main():
    kind = None
    rows = {"languages": [], "forms": []}
    for line in CATALOG.read_text(encoding="utf-8").splitlines():
        s = line.strip()
        if s.startswith("## Languages"):
            kind = "languages"
            continue
        if s.startswith("## Forms"):
            kind = "forms"
            continue
        if s.startswith("##"):
            kind = None
            continue
        if kind is None or not s.startswith("|"):
            continue
        cells = parse_cells(line)
        if (len(cells) != len(COLS) or cells[0] == "Name"
                or is_separator(cells)):
            continue
        row = dict(zip(COLS, cells))
        check_vocabulary(row)
        rows[kind].append(row)

    parts = [
        "-- SPDX-License-Identifier: Apache-2.0",
        "-- Copyright 2026 Dana Schlifka",
        "--",
        "-- GENERATED from additional_docs/lat_catalog.md by tools/gen_seed.py.",
        "-- Do not edit by hand; edit the catalogue and regenerate.",
        "-- Applied on first start when the tables are empty.",
        "-- tags/themes are JSON arrays.",
        "",
    ]
    for table in ("languages", "forms"):
        parts.append(
            f"INSERT INTO {table}\n"
            "    (name, description, focus, category, classification, feature,\n"
            "     forced_choice, attachment, tags, themes)\n"
            "VALUES"
        )
        values = [
            "    ("
            + ", ".join([
                sql_str(d["name"]),
                sql_str(d["description"]),
                sql_str(d["focus"]),
                sql_str(d["category"]),
                sql_str(d["classification"]),
                sql_str(d["feature"]),
                sql_str(d["forced_choice"]),
                sql_str(d["attachment"]),
                sql_str(json_arr(d["tags"])),
                sql_str(json_arr(d["themes"])),
            ])
            + ")"
            for d in rows[table]
        ]
        parts.append(",\n".join(values) + ";\n")

    OUT.write_text("\n".join(parts), encoding="utf-8")
    print(f"wrote {OUT.relative_to(REPO)}: "
          f"{len(rows['languages'])} languages, {len(rows['forms'])} forms")
    return 0


if __name__ == "__main__":
    sys.exit(main())
