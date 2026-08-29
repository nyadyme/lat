#!/usr/bin/env python
# SPDX-License-Identifier: Apache-2.0
# Copyright 2026 Dana Schlifka
"""Generate the Gemini and Copilot prompts from the Claude skill.

`.claude/skills/reframe-through-structure/SKILL.md` is the single source of
truth. Run this after editing it:

    python tools/gen_agent_prompts.py

Writes:
    .gemini/commands/reframe-through-structure.toml
    .github/prompts/reframe-through-structure.prompt.md

Use --check to verify the outputs are current without writing (exit 1 if not);
that is the form to run in CI.

The three files differ only in platform adaptations, all of them declared in
ADAPTATIONS below: tool names lose the `mcp__lat__` prefix, the self-reference
changes ("this skill" / "this command" / "this prompt"), Gemini gains an
`## Input` section carrying its `{{args}}` placeholder, and Copilot's first
workflow step mentions the editor selection. Everything else is copied verbatim,
so a substantive edit belongs in SKILL.md and nowhere else.
"""
import argparse
import re
import sys
from pathlib import Path

REPO = Path(__file__).resolve().parent.parent
SKILL = REPO / ".claude" / "skills" / "reframe-through-structure" / "SKILL.md"
GEMINI = REPO / ".gemini" / "commands" / "reframe-through-structure.toml"
COPILOT = REPO / ".github" / "prompts" / "reframe-through-structure.prompt.md"

WIDTH = 80

# One-line summary for the two derivative front matters. Deliberately not taken
# from the skill's own `description`, which is a trigger list tuned to Claude's
# skill dispatch and reads poorly as a command description.
SUMMARY = ("Reframe a text through a contrasting language or bound form "
           "(via the lat MCP) to surface hidden structural assumptions.")

# Substitutions applied to the skill body, in order. Matching is
# whitespace-insensitive, so a pattern may span the skill's line breaks.
COMMON = [
    # Gemini and Copilot address the lat tools without the MCP name prefix.
    ("mcp__lat__", ""),
    ("The pattern catalogue comes from the `lat` MCP server.",
     "The pattern catalogue comes from the **lat** MCP server."),
    # Without that prefix "the MCP" is ambiguous, and these hosts need a pointer
    # to the setup instructions.
    ("If the MCP is not connected", "If lat is not connected"),
    ("Minnesang, Haiku, Enumeration; the techniques).",
     "Minnesang, Haiku, Enumeration; the techniques); see the repository README "
     "for setup."),
]

ADAPTATIONS = {
    "gemini": COMMON + [("This skill uses", "This command uses")],
    "copilot": COMMON + [
        ("This skill uses", "This prompt uses"),
        ("Get the concrete text/sentence/thought. If none is given, ask for a "
         "passage or a clear core statement.",
         "Get the concrete text/sentence/thought. If none is given (or nothing "
         "is selected in the editor), ask for a passage or a clear core "
         "statement."),
    ],
}

# Gemini commands receive their argument through this placeholder; there is no
# equivalent in the skill, so the section is inserted rather than transformed.
GEMINI_INPUT = """## Input

If an argument was given, treat it as the text to reframe:
{{args}}
If it is empty, ask the user for a concrete sentence or paragraph.
"""

COPILOT_FRONTMATTER = f"""---
mode: agent
description: {SUMMARY}
---

<!-- GENERATED from .claude/skills/reframe-through-structure/SKILL.md by
     tools/gen_agent_prompts.py. Do not edit by hand; edit the skill and
     regenerate. -->"""

GEMINI_HEADER = """# GENERATED from .claude/skills/reframe-through-structure/SKILL.md
# by tools/gen_agent_prompts.py.
# Do not edit by hand; edit the skill and regenerate.
"""

LIST_MARKER = re.compile(r"^\s*(?:[-*]|\d+\.)\s")


def tokenize(content):
    """Split into wrappable tokens, keeping inline code spans whole.

    A code span may contain spaces (`Perspective & reciprocity`), so words are
    merged until the backtick count balances; a line break inside a span would
    otherwise split a term the reader is meant to match against the catalogue.
    """
    tokens, pending, inside = [], [], False
    for word in content.split():
        pending.append(word)
        if word.count("`") % 2:
            inside = not inside
        if not inside:
            tokens.append(" ".join(pending))
            pending = []
    if pending:                      # unbalanced backticks: emit what is left
        tokens.append(" ".join(pending))
    return tokens


def strip_frontmatter(text):
    """Return the skill body without its YAML front matter."""
    match = re.match(r"^---\n.*?\n---\n(.*)$", text, re.S)
    if not match:
        raise SystemExit(f"{SKILL}: no YAML front matter found")
    return match.group(1).lstrip("\n")


def split_units(block):
    """Split a block into list items (marker + continuations) or one paragraph.

    Rewrapping has to happen per item, not per block: the example section is a
    single blank-line-free list whose sub-bullets must stay one line each.
    """
    if not LIST_MARKER.match(block.split("\n")[0]):
        return [block]
    units, current = [], []
    for line in block.split("\n"):
        if LIST_MARKER.match(line) and current:
            units.append("\n".join(current))
            current = []
        current.append(line)
    if current:
        units.append("\n".join(current))
    return units


def wrappable(unit):
    """False for units that must keep their hand-set line breaks.

    Tables, headings and the args placeholder are structural. A long inline code
    span is an indivisible token — the example's tool calls run past the margin
    on purpose, and reflowing them would only move the overflow.
    """
    stripped = unit.lstrip()
    if stripped.startswith(("|", "#", ">")) or unit.startswith("    "):
        return False
    if "{{args}}" in unit:
        return False
    return not any(len(span) > 40 for span in re.findall(r"`[^`]*`", unit))


def flatten(text):
    """Collapse to one line, so a substitution can span the source line breaks."""
    return " ".join(text.split())


def rewrap(unit, content):
    """Lay `content` out under the marker and indentation taken from `unit`."""
    first = unit.split("\n")[0]
    indent = re.match(r"^\s*", first).group(0)
    found = re.match(r"^\s*((?:[-*]|\d+\.)\s+)", first)
    marker = found.group(1) if found else ""
    hanging = indent + " " * len(marker)
    if marker:
        content = content[len(marker.strip()):].lstrip()

    lines, current, started = [], indent + marker, False
    for token in tokenize(content):
        candidate = current + (" " if started else "") + token
        if started and len(candidate) > WIDTH:
            lines.append(current)
            current = hanging + token
        else:
            current = candidate
        started = True
    lines.append(current)
    return "\n".join(lines)


def transform(body, target):
    """Apply the target's substitutions and rewrap only what they changed.

    An untouched unit is copied through byte for byte, so the diff between the
    skill and a derivative stays limited to the declared adaptations.
    """
    subs = [(flatten(old), flatten(new)) for old, new in ADAPTATIONS[target]]
    out = []
    for block in body.split("\n\n"):
        pieces = []
        for unit in split_units(block):
            flat = flatten(unit)
            changed = flat
            for old, new in subs:
                changed = changed.replace(old, new)
            if changed == flat:
                pieces.append(unit)
            elif wrappable(unit):
                pieces.append(rewrap(unit, changed))
            else:
                # Keep the hand-set breaks; substitute within each line.
                for old, new in ADAPTATIONS[target]:
                    unit = unit.replace(old, new)
                pieces.append(unit)
        out.append("\n".join(pieces))
    return "\n\n".join(out)


def insert_after_section(body, heading, section):
    """Insert `section` as a new block directly after the named section."""
    blocks = body.split("\n\n")
    for i, block in enumerate(blocks):
        if block.strip().startswith(heading):
            j = i + 1
            while j < len(blocks) and not blocks[j].lstrip().startswith("## "):
                j += 1
            blocks.insert(j, section.strip())
            return "\n\n".join(blocks)
    raise SystemExit(f"section {heading!r} not found in the skill")


def render_gemini(body):
    text = transform(body, "gemini")
    text = insert_after_section(text, "## Prerequisite", GEMINI_INPUT)
    if '"""' in text:
        raise SystemExit("gemini: body contains a TOML string delimiter")
    if chr(92) in text:
        raise SystemExit("gemini: backslash would be a TOML escape sequence")
    return (GEMINI_HEADER + '\ndescription = "' + SUMMARY + '"\n\n'
            'prompt = """\n' + text.rstrip() + '\n"""\n')


def render_copilot(body):
    return COPILOT_FRONTMATTER + "\n\n" + transform(body, "copilot").rstrip() + "\n"


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--check", action="store_true",
                        help="verify the outputs are current; do not write")
    args = parser.parse_args()

    body = strip_frontmatter(SKILL.read_text(encoding="utf-8"))
    targets = [(GEMINI, render_gemini(body)), (COPILOT, render_copilot(body))]

    stale = []
    for path, content in targets:
        current = path.read_text(encoding="utf-8") if path.exists() else None
        if args.check:
            if current != content:
                stale.append(path)
            continue
        if current != content:
            path.write_text(content, encoding="utf-8")
            print(f"wrote {path.relative_to(REPO)}")
        else:
            print(f"unchanged {path.relative_to(REPO)}")

    if args.check:
        for path in stale:
            print(f"STALE {path.relative_to(REPO)}", file=sys.stderr)
        if stale:
            print("run: python tools/gen_agent_prompts.py", file=sys.stderr)
            return 1
        print(f"up to date: {GEMINI.name}, {COPILOT.name}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
