#!/usr/bin/env python
# SPDX-License-Identifier: Apache-2.0
# Copyright 2026 Dana Schlifka
"""Generate additional_docs/lat_facets.md from additional_docs/lat_catalog.md.

Facets are the distinct filter values per table (what the `list_facets` tool
returns). Since the database is generated from the catalogue, computing facets
directly from the catalogue yields the identical result and needs no running DB.

    python tools/gen_facets.py
"""
import sys
from pathlib import Path

REPO = Path(__file__).resolve().parent.parent
CATALOG = REPO / "additional_docs" / "lat_catalog.md"
OUT = REPO / "additional_docs" / "lat_facets.md"

COLS = ["name", "category", "classification", "focus", "feature",
        "description", "tags", "themes"]


def parse_cells(line):
    inner = line.strip()
    if inner.startswith("|"):
        inner = inner[1:]
    if inner.endswith("|"):
        inner = inner[:-1]
    return [c.strip() for c in inner.split("|")]


def is_separator(cells):
    return all(c and set(c) <= set("-: ") for c in cells)


def parse_catalog():
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
        if len(cells) != 8 or cells[0] == "Name" or is_separator(cells):
            continue
        rows[kind].append(dict(zip(COLS, cells)))
    return rows


def distinct_col(rows, col):
    return sorted({r[col] for r in rows if r[col]})


def distinct_arr(rows, col):
    vals = set()
    for r in rows:
        vals.update(x.strip() for x in r[col].split(",") if x.strip())
    return sorted(vals)


def bullets(items):
    return "\n".join(f"- {x}" for x in items)


def tagline(items):
    return ", ".join(f"`{x}`" for x in items)


def main():
    rows = parse_catalog()
    lines = [
        "# lat – Facets (generated snapshot)",
        "",
        "Distinct filter values actually present in the seeded database, per table —",
        "i.e. what `list_facets` returns. GENERATED from additional_docs/lat_catalog.md",
        "by tools/gen_facets.py; regenerate after changing the catalogue. `themes` is the",
        "closed cognitive-axis vocabulary; `tags` are free keywords.",
        "",
        "> Note the asymmetry: **forms carry 9 themes, languages 10** — `Possession & belonging`",
        "> appears only among languages (Navajo inalienable possession, Dyirbal), so it is",
        "> absent from the forms facet. Facets reflect the occupied subset, not the abstract list.",
        "",
    ]
    for table, title in (("languages", "Languages"), ("forms", "Forms")):
        data = rows[table]
        themes = distinct_arr(data, "themes")
        cats = distinct_col(data, "category")
        classes = distinct_col(data, "classification")
        tags = distinct_arr(data, "tags")
        lines += [
            f"## {title} ({len(data)} entries)",
            "",
            f"### Themes ({len(themes)})",
            bullets(themes),
            "",
            f"### Categories ({len(cats)})",
            bullets(cats),
            "",
            f"### Classifications ({len(classes)})",
            bullets(classes),
            "",
            f"### Tags ({len(tags)})",
            tagline(tags),
            "",
        ]

    OUT.write_text("\n".join(lines), encoding="utf-8")
    print(f"wrote {OUT.relative_to(REPO)}: "
          f"languages themes={len(distinct_arr(rows['languages'], 'themes'))}, "
          f"forms themes={len(distinct_arr(rows['forms'], 'themes'))}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
