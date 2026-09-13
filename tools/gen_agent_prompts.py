#!/usr/bin/env python
# SPDX-License-Identifier: Apache-2.0
# Copyright 2026 Dana Schlifka
"""Generate the Gemini command and the Copilot prompt from the Claude skill.

`.claude/skills/reframe-through-structure/SKILL.md` is the single source of
truth for the workflow. Run this after editing it:

    python tools/gen_agent_prompts.py

Both outputs are rewritten in full, so never edit them by hand. Pass --check to
compare them against what the skill would produce and exit 1 if either has
fallen behind, without writing anything — for a build that must not silently
ship a stale copy.

What differs between the three hosts is small and deliberate: Claude prefixes
the MCP tools with `mcp__lat__` and the other two do not, each host calls the
workflow by a different noun, and each receives its argument differently.
Everything else is the skill text verbatim, line breaks included. A rewrite
re-wraps only the unit it touched, and only when that unit outgrew the line
limit, so the generated files stay diffable against their source.
"""
import re
import sys
from pathlib import Path

REPO = Path(__file__).resolve().parent.parent
SKILL = REPO / ".claude" / "skills" / "reframe-through-structure" / "SKILL.md"
GEMINI = REPO / ".gemini" / "commands" / "reframe-through-structure.toml"
COPILOT = REPO / ".github" / "prompts" / "reframe-through-structure.prompt.md"

WIDTH = 80

# One line, shown in both hosts' command pickers. The skill's own front-matter
# description is written for Claude's skill matcher — several sentences of
# trigger phrases — and is far too long for a picker entry, so it is not
# derived from it.
SUMMARY = ("Reframe a text through a contrasting language or bound form "
           "(via the lat MCP) to surface hidden structural assumptions.")

# Literal substrings, not patterns. Every anchor below occurs in exactly one
# source line (the tool prefix always means the same thing wherever it stands),
# and a literal cannot mis-fire on text added to the skill later. An
# anchor that stops matching aborts the run — see `substitute`.
SHARED = [
    ("`lat` MCP server", "**lat** MCP server"),
    ("mcp__lat__", ""),
    ("If the MCP is not connected", "If lat is not connected"),
    ("Enumeration; the techniques).",
     "Enumeration; the techniques); see the repository README for setup."),
]

GEMINI_REWRITES = [("This skill uses", "This command uses")]

COPILOT_REWRITES = [
    ("This skill uses", "This prompt uses"),
    # Copilot runs against the editor, where a selection is the usual input.
    ("given, ask for a passage",
     "given (or nothing is selected in the editor), ask for a passage"),
]

# Gemini passes the invocation's argument as {{args}}. The skill carries no
# such placeholder because Claude takes the text from the conversation, so the
# section is inserted rather than rewritten.
GEMINI_INPUT = """## Input

If an argument was given, treat it as the text to reframe:
{{args}}
If it is empty, ask the user for a concrete sentence or paragraph.

"""

# Lines that are never re-wrapped and never merged with a neighbour: headings,
# table rows, block quotes, fence markers.
KEEP = re.compile(r"^(?:#|\||>|```)")

# A list item, with its bullet or number and the space after it.
ITEM = re.compile(r"^(\s*)(?:[-*+]|\d+\.)\s+")


def body_of(text):
    """The skill text after its front matter, stripped of blank edges."""
    if not text.startswith("---\n"):
        raise SystemExit(f"{SKILL}: no front matter")
    end = text.find("\n---\n", 4)
    if end < 0:
        raise SystemExit(f"{SKILL}: front matter is not closed")
    return text[end + len("\n---\n"):].strip("\n")


def units(lines):
    """Group lines into units that may be re-wrapped on their own.

    A unit is one paragraph, or one list item together with its continuation
    lines, or a single line that is never re-wrapped at all. The grouping is
    what keeps a re-wrap from pulling words across a blank line or across the
    boundary between two list items.
    """
    buf = []
    for line in lines:
        if not line.strip() or KEEP.match(line) or ITEM.match(line):
            if buf:
                yield buf
                buf = []
        if not line.strip() or KEEP.match(line):
            yield [line]
            continue
        buf.append(line)
    if buf:
        yield buf


def leading(line):
    """The whitespace a line starts with."""
    return line[:len(line) - len(line.lstrip())]


def indents(unit):
    """The prefix of the unit's first line, and of every following one."""
    item = ITEM.match(unit[0])
    first = item.group(0) if item else leading(unit[0])
    if len(unit) > 1:
        rest = leading(unit[1])
    else:
        # A one-line item has no continuation to copy the indent from; align
        # any new line under the item's text rather than under its bullet.
        rest = " " * len(first)
    return first, rest


def atoms(text):
    """Split on whitespace, but never inside a backticked span.

    `` `search_patterns { "kind": "form" }` `` carries spaces and must survive
    as one token: broken across a line it stops being code, and in the example
    calls it stops being valid JSON. A span that does not fit is emitted alone
    and allowed to run past the limit, which is what the skill does by hand.
    """
    out = []
    current = ""
    in_code = False
    for char in text:
        if char == "`":
            in_code = not in_code
        if char.isspace() and not in_code:
            if current:
                out.append(current)
                current = ""
            continue
        current += char
    if current:
        out.append(current)
    return out


def rewrap(unit):
    """Re-flow one unit greedily to `WIDTH`, keeping its indentation."""
    first, rest = indents(unit)
    # The bullet or number is re-attached as the first line's indent, so it has
    # to come off that line's text or it would be written twice.
    body = [unit[0][len(first):]] + list(unit[1:])
    lines = []
    current = first
    empty = True
    for atom in atoms(" ".join(line.strip() for line in body)):
        candidate = current + ("" if empty else " ") + atom
        if not empty and len(candidate) > WIDTH:
            lines.append(current)
            current = rest + atom
        else:
            current = candidate
        empty = False
    lines.append(current)
    return lines


def substitute(body, rewrites):
    """Apply the rewrites, re-flowing only a unit that outgrew the line limit.

    Every anchor sits inside a single source line, so a substitution needs no
    context. A unit then keeps the skill's own line breaks unless a rewrite
    pushed one of its lines past `WIDTH` — which in practice only the host's
    name for itself does, since every other rewrite shortens.

    Re-flowing more than that is tempting and wrong: it would rewrap paragraphs
    no rewrite touched, so an unrelated edit to the skill would churn the whole
    file and bury the real change in the diff.
    """
    seen = set()
    out = []
    for unit in units(body.splitlines()):
        new = []
        for line in unit:
            for old, replacement in rewrites:
                if old in line:
                    seen.add(old)
                    line = line.replace(old, replacement)
            new.append(line)
        if len(new) == 1 and (not new[0].strip() or KEEP.match(new[0])):
            out.append(new[0])
            continue
        outgrew = any(len(n) > WIDTH and len(n) > len(o)
                      for n, o in zip(new, unit))
        out.extend(rewrap(new) if outgrew else new)

    missing = [old for old, _ in rewrites if old not in seen]
    if missing:
        # Silence here would ship a host file still naming Claude's tools or
        # calling itself a skill, and nothing downstream would notice.
        raise SystemExit(
            f"{SKILL}: these anchors no longer match, so the rewrite they "
            f"stand for was skipped: {missing}")
    return "\n".join(out)


def gemini_toml(body):
    """Wrap the body as a Gemini custom command."""
    body = body.replace("## Workflow\n", GEMINI_INPUT + "## Workflow\n", 1)
    if '"""' in body:
        raise SystemExit(
            "the skill contains a triple quote, which would end the TOML "
            "string early; rephrase it or escape it here")
    return (
        "# GENERATED from .claude/skills/reframe-through-structure/SKILL.md\n"
        "# by tools/gen_agent_prompts.py.\n"
        "# Do not edit by hand; edit the skill and regenerate.\n"
        "\n"
        f'description = "{SUMMARY}"\n'
        "\n"
        'prompt = """\n'
        f"{body}\n"
        '"""\n'
    )


def copilot_prompt(body):
    """Wrap the body as a Copilot agent prompt file."""
    return (
        "---\n"
        "mode: agent\n"
        f"description: {SUMMARY}\n"
        "---\n"
        "\n"
        "<!-- GENERATED from .claude/skills/reframe-through-structure/SKILL.md"
        " by\n"
        "     tools/gen_agent_prompts.py. Do not edit by hand; edit the skill"
        " and\n"
        "     regenerate. -->\n"
        "\n"
        f"{body}\n"
    )


def main(argv):
    unknown = [a for a in argv[1:] if a != "--check"]
    if unknown:
        raise SystemExit(f"unknown argument(s): {unknown}; only --check")
    check = "--check" in argv[1:]

    body = body_of(SKILL.read_text(encoding="utf-8"))
    built = {
        GEMINI: gemini_toml(substitute(body, SHARED + GEMINI_REWRITES)),
        COPILOT: copilot_prompt(substitute(body, SHARED + COPILOT_REWRITES)),
    }

    if check:
        # newline="" so the comparison sees the bytes on disk rather than a
        # platform translation of them.
        stale = [path.relative_to(REPO) for path, text in built.items()
                 if not path.exists()
                 or path.read_text(encoding="utf-8", newline="") != text]
        for path in stale:
            print(f"{path} is behind the skill; "
                  "run tools/gen_agent_prompts.py", file=sys.stderr)
        if stale:
            return 1
        print(f"{len(built)} generated copies are current")
        return 0

    for path, text in built.items():
        path.write_text(text, encoding="utf-8", newline="\n")
        print(f"wrote {path.relative_to(REPO)}")
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
