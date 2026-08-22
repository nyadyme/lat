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
        "description", "tags", "themes", "consumes", "produces"]


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
        if len(cells) != 10 or cells[0] == "Name" or is_separator(cells):
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


def carriers(rows, theme):
    """Names of the entries whose themes cell contains theme, in file order."""
    return [r["name"] for r in rows
            if theme in {x.strip() for x in r["themes"].split(",")}]


def wrap_quote(text, width=88):
    """Wrap text into blockquote lines, each prefixed with '> '."""
    lines, current = [], ""
    for word in text.split():
        candidate = f"{current} {word}".strip()
        if current and len(candidate) + 2 > width:
            lines.append(f"> {current}")
            current = word
        else:
            current = candidate
    if current:
        lines.append(f"> {current}")
    return lines


def asymmetry_note(rows):
    """Build the blockquote about themes occupied by only one of the tables.

    Derived from the catalogue rather than stated, so it cannot go stale: which
    axis is exclusive to which table, and which entries carry it, are read off
    the rows on every run.
    """
    occupied = {t: set(distinct_arr(rows[t], "themes")) for t in rows}
    exclusive = {
        table: sorted(themes - set().union(
            *(o for t, o in occupied.items() if t != table)))
        for table, themes in occupied.items()
    }
    counts = ", ".join(f"{t} {len(o)}" for t, o in sorted(occupied.items()))

    if not any(exclusive.values()):
        sentences = [f"Themes occupied per table: **{counts}** — the tables "
                     "cover the same axes."]
    else:
        sentences = [f"Note the asymmetry: **themes occupied per table are "
                     f"{counts}**."]
        others = {t: sorted(set(rows) - {t}) for t in rows}
        for table, themes in sorted(exclusive.items()):
            for theme in themes:
                names = ", ".join(carriers(rows[table], theme))
                absent = " and ".join(others[table])
                sentences.append(
                    f"`{theme}` appears only among {table} — carried by "
                    f"{names} — so it is absent from the {absent} facet.")

    sentences.append("Facets reflect the occupied subset, not the abstract "
                     "list.")
    return wrap_quote(" ".join(sentences))


def main():
    rows = parse_catalog()
    lines = [
        "# lat – Facets (generated snapshot)",
        "",
        "Distinct filter values actually present in the seeded database, per table —",
        "i.e. what `list_facets` returns. GENERATED from additional_docs/lat_catalog.md",
        "by tools/gen_facets.py; regenerate after changing the catalogue. `themes` is the",
        "closed cognitive-axis vocabulary; `tags` are free keywords; `consumes`/`produces`",
        "are the closed stack-type vocabulary — a lens whose `produces` meets another's",
        "`consumes` is a serial pair.",
        "",
        *asymmetry_note(rows),
        "",
    ]
    for table, title in (("languages", "Languages"), ("forms", "Forms")):
        data = rows[table]
        themes = distinct_arr(data, "themes")
        cats = distinct_col(data, "category")
        classes = distinct_col(data, "classification")
        tags = distinct_arr(data, "tags")
        consumes = distinct_arr(data, "consumes")
        produces = distinct_arr(data, "produces")
        lines += [
            f"## {title} ({len(data)} entries)",
            "",
            f"### Themes ({len(themes)})",
            bullets(themes),
            "",
            f"### Consumes ({len(consumes)})",
            tagline(consumes),
            "",
            f"### Produces ({len(produces)})",
            tagline(produces),
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
