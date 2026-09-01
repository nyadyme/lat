# lat — Language as a Tool

[![Software DOI (all versions)](https://zenodo.org/badge/DOI/10.5281/zenodo.21508868.svg)](https://doi.org/10.5281/zenodo.21508868)

*Concept DOI of the software — it always resolves to the newest release. The
founding essay is a separate work with its own DOI (see below).*

`lat` is a small, local [MCP](https://modelcontextprotocol.io) server written in
Rust. It hands an AI agent a curated catalog of thinking patterns —
natural languages (grammatical reflexes) and bound poetic forms / writing
techniques — that each force a different structure of thought. The agent can
look up a pattern that fits a problem and then reformulate a text through it to
surface assumptions the original phrasing kept hidden.

The idea: language quietly imposes patterns and structures on whatever it
describes. Forcing the content of a text through a structure foreign to it
(e.g. Russian *aspect*, the Japanese *wa* topic, a Haiku's cut) works like a
change of lighting: the same object, different shadows. It rotates the input
like a cube, so the same object can be viewed from different grammatical
angles. The goal is not the "correct" language but mobility.
The founding essay *Language as a Tool of Thought* is
not bundled in this repository; it is deposited as the canonical, citable
version on Zenodo:
[10.5281/zenodo.21382455](https://doi.org/10.5281/zenodo.21382455).

The server speaks standard MCP over stdio or streamable HTTP, so it works with
any MCP-capable host. This project documents and supports Claude, Google
Gemini, and GitHub Copilot.

As a defensive prior-art disclosure, the method is stated here in general
terms. Given any natural-language text, the toolchain performs context-aware
*linguistic contrast-routing* — an inverted soft Sapir–Whorf model, see
[Theoretical framing](#theoretical-framing): (1) it analyzes the grammatical bias
of the input language (e.g., the causal-chain / agent-object bias of German);
(2) maps that bias to one of a closed set of cognitive axes; (3) selects one or more
structurally orthogonal patterns — either a different natural language
(e.g., Japanese topic-prominence, Yucatec Maya noun-classification) or a bound
poetic / formal structure — from a multilingual catalog, excluding the input
language so a contrasting lens is surfaced; and (4) reformulates the text to
expose assumptions that are artifacts of the source grammar rather than inherent
to the subject (for example, metaphysical assumptions in scientific texts). This
selection and reformulation may be performed automatically by a software agent
or interactively; both embodiments are disclosed.

The linguistic findings this builds on are not the author's work (see Lucy,
Slobin, and the linguistic-relativity literature in general); what is disclosed
is the automated synthesis — the routing of a text's structural bias to
orthogonal patterns from a curated catalog.

See the concrete steps under [Usage / invocation](#usage--invocation), the
prior-art statement in [NOTICE](NOTICE), and the dated deposit at
[doi:10.5281/zenodo.21382455](https://doi.org/10.5281/zenodo.21382455).



## Contents

- [Theoretical framing](#theoretical-framing)
- [How it fits together](#how-it-fits-together)
- [Data model](#data-model)
- [Tools](#tools)
- [Build](#build)
- [Integration](#integration) — Claude, Gemini, Copilot
- [Usage / invocation](#usage--invocation)
- [Database location](#database-location)
- [Terminology](#terminology)
- [Editing the catalog](#editing-the-catalog)
- [Project layout](#project-layout)
- [License](#license)

## Theoretical framing

Context-aware *linguistic contrast-routing* is an inverted soft Sapir–Whorf
model.

Soft — the weak reading of linguistic relativity: a language does not
determine what can be thought. It only makes some distinctions obligatory
(Russian cannot state an action without committing to its aspect; Japanese *wa*
forces a decision about what the topic is) and lets others stay comfortably
implicit. That is a bias, not a prison — no claim of untranslatability or of
thought being bounded by vocabulary is made or needed here.

Inverted — the classical reading takes that bias as a given condition of the
speaker and asks what a language *does to* its thinking. This tool reverses the
direction of use: the bias becomes an instrument that is *chosen*. A structure is
selected precisely because it forces the distinction the source language leaves
implicit, and the text is deliberately run through it.

So the relativity effect is not a defect to be corrected but the mechanism the
tool runs on. Whatever differs between the original and the
reformulation is precisely what was an artifact of the source grammar rather
than a property of the subject. This is also why the input language is excluded
(`exclude_names`): without contrast there is nothing to read off, and the
strength of the reading scales with the structural distance of the lens — hence
the closed set of cognitive axes rather than an arbitrary pick.

## How it fits together

| Piece | Role |
|---|---|
| `lat` MCP server (this crate) | The data layer: a read-only catalog of patterns, queryable over MCP. |
| `reframe-through-structure` workflow | Diagnoses a text's structural bias, queries the server, and applies the reformulation techniques. Bundled for Claude (skill), Copilot (prompt file) and Gemini (command). |

The server is agent-agnostic — any MCP-compatible client can use it.

## Data model

Two SQLite tables, `forms` (poetic forms / writing techniques) and `languages`,
share the same columns:

| Column | Meaning |
|---|---|
| `name` | Unique identifier, e.g. `Haiku`, `Russian` |
| `description` | What the pattern does |
| `focus` | What it brings into focus (e.g. *manner of unfolding*) |
| `category` | e.g. `Poetic form`, `Register`, `Technique`, `Language` |
| `classification` | e.g. `aspect language`, `classifier language`, `bound form` |
| `feature` | The distinction it makes obligatory |
| `forced_choice` | The choice it makes obligatory, phrased as a question. Deliberately **not** unique |
| `attachment` | The constituent it interrogates. Closed vocabulary (see below) |
| `tags` | JSON array of free keywords (linguistic features, formal mechanics) |
| `themes` | JSON array from a closed cognitive-axis vocabulary (see below) |

`themes` is drawn from a fixed set of eleven axes — `Causality`,
`Agency & control`, `Rank & salience`, `Time & aspect`, `Coexistence`,
`Perspective & reciprocity`, `Object boundaries`, `Evidence & certainty`,
`Space & orientation`, `Possession & belonging`,
`Logic & ambiguity` — used to match a problem's structural bias to patterns that
reframe that axis. Every language carries a theme and so do most forms; only
patterns whose mechanics are purely acoustic, metrical or typographic carry none,
and those are found via `tags`/`category`. Multi-valued `tags` and `themes`
are stored as JSON arrays and filtered with SQLite's `json_each` (guarded by
`json_valid`, so a malformed cell cannot abort a query).

`forced_choice` and `attachment` are the pair that decides whether two patterns
may be combined. `attachment` is drawn from eleven anchors — `verb`, `subject`,
`object`, `noun`, `possessive`, `person`, `spatial frame`, `connective`,
`word order`, `whole passage`, `surface` — and an unknown value aborts seed
generation. `forced_choice` is written to be *shared*: two patterns that force
the same choice carry byte-identical strings, so equality of the pair means
their findings are correlated and a combination should take at most one of them.
That is what `focus` cannot do — it is authored per entry to be distinctive, so
Tuyuca and Tariana describe one and the same choice in two different phrasings.
The colliding groups are listed, generated from the catalog, in
[`additional_docs/lat_facets.md`](additional_docs/lat_facets.md).

The catalog is maintained in
[`additional_docs/lat_catalog.md`](additional_docs/lat_catalog.md) (single source
of truth) and `src/seed.sql` is generated from it. Both are covered by the
repository's [Apache-2.0](LICENSE) license, like the code — see
[License](#license).

## Tools

All tools are read-only. Every filter is optional and filters are AND-combined.

| Tool | Purpose |
|---|---|
| `search_patterns` | Find patterns by `kind`, `theme`, `category`, `classification`, `focus`, `forced_choice`, `attachment`, `tag`, free `text`, or `exclude_names` (drop the user's own language so contrasting lenses surface). |
| `get_pattern` | Full details of one pattern by `kind` + `name`. |
| `list_patterns` | List all patterns, optionally restricted to one `kind`. |
| `list_facets` | Distinct categories / classifications / attachments / tags / themes per table, so the agent knows valid filter values. |

`kind` is `form` or `language` (omit it to search both).

## Build

Prerequisites: a Rust toolchain with edition 2024 support (Rust 1.85+).
SQLite is compiled in via the `bundled` feature — no system SQLite needed.

```sh
git clone https://github.com/nyadyme/lat
cd lat
cargo build --release
```

The binary is written to:

- Windows: `target\release\lat.exe`
- macOS / Linux: `target/release/lat`

Note the absolute path to that binary — every client integration below needs
it. A quick self-test over stdio (optional):

```sh
printf '%s\n' \
  '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05","capabilities":{},"clientInfo":{"name":"probe","version":"0"}}}' \
  '{"jsonrpc":"2.0","method":"notifications/initialized"}' \
  '{"jsonrpc":"2.0","id":2,"method":"tools/list"}' \
  | ./target/release/lat
```

You should see the four tools listed. Logs go to stderr; stdout carries
the MCP JSON-RPC stream — never print anything else to stdout.

## Transports

`lat` speaks two transports over the same four tools:

| Transport | Invocation | Use it when |
| --- | --- | --- |
| stdio (default) | `lat` | The host launches the binary as a child process. |
| streamable HTTP | `lat --http [ADDR]` | The host connects to a URL — Claude Desktop custom connectors, a shared or remote instance. |

```sh
lat                        # stdio
lat --http                 # http://127.0.0.1:8000/mcp
lat --http 127.0.0.1:9000  # explicit address
lat --help                 # all flags and environment variables
```

The MCP endpoint is `/mcp`; `GET /health` returns `ok` for supervisors. Ctrl-C
shuts the server down gracefully.

Host validation. Only loopback `Host` headers are accepted by default, which
protects a locally bound server against DNS rebinding. If you bind to a real
hostname, list it in `LAT_HTTP_ALLOWED_HOSTS` (comma-separated; loopback stays
allowed) — otherwise those requests get `403`.

Exposure. The default bind is loopback only. Anything beyond that is a
public MCP endpoint: `lat` has no authentication and no TLS of its own, so put a
reverse proxy in front of it if it leaves the machine.

## Integration

By default `lat` is a local stdio MCP server: each client is told the
absolute path to the `lat` binary and launches it as a child process. On first
launch the server creates and seeds its database (see
[Database location](#database-location)), so it works from any client and any
working directory.

In every config below, replace the path with your own absolute path. On Windows,
JSON requires escaped backslashes (`C:\\Users\\…`) or forward slashes
(`C:/Users/…`). To pin the database explicitly, add an `env` block with
`LAT_DB_PATH` (shown once under Claude Desktop; the same key works everywhere).

### Claude Code (CLI)

Register at user scope so it is available in every project:

```sh
claude mcp add --scope user lat -- /absolute/path/to/target/release/lat
```

Verify / manage:

```sh
claude mcp list          # health check across servers
claude mcp get lat       # show the lat entry
claude mcp remove lat -s user
```

### Claude Desktop

Edit the config file:

- Windows: `%APPDATA%\Claude\claude_desktop_config.json`
- macOS: `~/Library/Application Support/Claude/claude_desktop_config.json`

```json
{
  "mcpServers": {
    "lat": {
      "command": "C:\\Users\\you\\path\\to\\target\\release\\lat.exe",
      "args": [],
      "env": {
        "LAT_DB_PATH": "C:\\Users\\you\\lat\\patterns.db"
      }
    }
  }
}
```

Restart Claude Desktop; the `lat` tools appear in the tool picker. (`env` is
optional — omit it to use the default database location.)

#### Claude Desktop over HTTP

Alternatively, run `lat` yourself and add it as a custom connector instead of
letting Claude Desktop launch it — useful when one running instance should serve
several clients, or when the server lives on another machine.

```sh
lat --http            # listens on http://127.0.0.1:8000/mcp
```

In Claude Desktop: Settings → Connectors → Add custom connector, then enter

```
http://127.0.0.1:8000/mcp
```

Keep the process running while you use it — with the connector, Claude Desktop
does not start or restart `lat` for you. For a non-loopback address, remember
`LAT_HTTP_ALLOWED_HOSTS` and read the exposure note under
[Transports](#transports).

### Google Gemini (Gemini CLI)

Add the server to `~/.gemini/settings.json`:

```json
{
  "mcpServers": {
    "lat": {
      "command": "/absolute/path/to/target/release/lat",
      "args": []
    }
  }
}
```

Recent Gemini CLI versions also expose a helper:

```sh
gemini mcp add lat /absolute/path/to/target/release/lat
```

Start `gemini`; the tools are discovered on launch (use `/mcp` to inspect
connected servers).

### GitHub Copilot (VS Code, Agent mode)

Create a workspace file `.vscode/mcp.json` (or use the MCP: Add Server
command for a user-level entry):

```json
{
  "servers": {
    "lat": {
      "type": "stdio",
      "command": "C:/Users/you/path/to/target/release/lat.exe",
      "args": []
    }
  }
}
```

Open Copilot Chat, switch to Agent mode, and the `lat` tools become
available to the model. (MCP support requires a current VS Code with Copilot;
Visual Studio and JetBrains AI assistants that speak MCP use an equivalent
`mcpServers` block.)

### Generic MCP host

Any MCP client that can launch a local stdio server works: point it at the
absolute path of the `lat` binary, no arguments, transport `stdio`. Optionally
set the `LAT_DB_PATH` environment variable.

Clients that speak streamable HTTP instead: start `lat --http` and point them
at `http://127.0.0.1:8000/mcp`. See [Transports](#transports).

### Supported clients

This project documents and supports Claude, Gemini, and Copilot. Because
`lat` implements the open MCP standard, it also works with other MCP-capable
hosts; those are simply not documented here.

## Usage / invocation

Once integrated, you do not call the tools directly — you ask the agent, and it
calls them. The intended loop (encoded in full by the companion skill) is:

1. Diagnose the structural bias of a text (causal chain? agent/object rank?
   fixed tense? tacit object boundaries?).
2. Map it to one of the eleven cognitive axes and `search_patterns` on that
   `theme`; use `list_facets` to see valid values.
3. Contrast: pass `exclude_names: ["<the language of the text>"]` so the
   structure the text already thinks in is not recommended back. Exclude exactly
   that one language — a closely related one stays a valid lens, since kinship is
   never uniform across the axes. Prefer lenses that foreground the axis the
   input language leaves implicit.
4. Reformulate the text through the chosen language/form and name what
   each version reveals.

### Example prompts

- "Reframe this paragraph through a linguistic structure that breaks its causal
  bias. Use the lat tools; I'm writing in English, so exclude English."
- "Which languages in the catalog foreground *aspect* or *evidentiality*? Use
  `search_patterns` on the matching themes."
- "Show me the available themes and categories" → the agent calls `list_facets`.
- "Give me the full entry for the Haiku" → `get_pattern { kind: form, name: "Haiku" }`.

### Direct tool arguments (reference)

```jsonc
// search by cognitive axis, excluding the language of the analyzed text
search_patterns { "kind": "language", "theme": "Time & aspect", "exclude_names": ["German"] }

// language-neutral forms for a given axis
search_patterns { "kind": "form", "theme": "Coexistence" }

// full detail
get_pattern { "kind": "language", "name": "Russian" }

// before combining: everything that forces the same choice at the same anchor
search_patterns { "attachment": "verb", "forced_choice": "the source of the information" }
```


### The packaged workflow

The workflow above ships as an invocable command for each supported client — all
named `reframe-through-structure`, all driving the same lat tools:

- Claude — a skill at `.claude/skills/reframe-through-structure/`. Copy it to
  your user skills directory to use it in every project:
  ```sh
  cp -r .claude/skills/reframe-through-structure ~/.claude/skills/
  ```
  Claude invokes it automatically when a request matches ("reframe this", "see
  this differently", …) or explicitly via `/reframe-through-structure`.
- GitHub Copilot — a prompt file at
  `.github/prompts/reframe-through-structure.prompt.md`. In Copilot Chat (Agent
  mode) run `/reframe-through-structure`.
- Gemini CLI — a custom command at
  `.gemini/commands/reframe-through-structure.toml`. Run
  `/reframe-through-structure` (optionally with the text to reframe as an
  argument).

All three require the lat MCP to be configured (see [Integration](#integration)).
Any other MCP host can reproduce the workflow by pasting the four steps above as
a system/instruction prompt.

## Database location

On first startup, the server creates and seeds the database if it is empty. The
path is resolved independently of the working directory:

1. `LAT_DB_PATH` environment variable, if set; otherwise
2. the platform data directory — `%APPDATA%\lat\patterns.db` on Windows,
   `~/Library/Application Support/lat/patterns.db` on macOS,
   `~/.local/share/lat/patterns.db` on Linux.

Seed data is embedded in the binary from [`src/seed.sql`](src/seed.sql) and is
only applied when both tables are empty. Seeding is atomic — an interrupted first
run rolls back rather than leaving a half-filled database.

A database written by an older build is not rebuilt: columns added since
(`forced_choice`, `attachment`) are appended in place, so live edits survive,
but their cells stay empty and the server logs a warning. Delete the database
file and restart to fill them from the catalog.

## Terminology

Some category labels in the catalog reproduce terms from the older linguistic
literature — *Eskimo*, for instance, rather than *Inuit* or *Yupik*. They are
retained for retrieval: the sources an agent is likely to draw on index the
material under these terms, and renaming the categories would break that match.
Their presence reflects the terminology of that literature, not an endorsement
of it.

## Editing the catalog

- Curated edits (recommended): edit `additional_docs/lat_catalog.md` (the
  single source of truth), then regenerate the seed and reseed:
  ```sh
  python tools/gen_seed.py      # regenerate src/seed.sql from the catalog
  python tools/gen_facets.py    # refresh additional_docs/lat_facets.md (filter-value reference)
  # then delete the database file and restart, or rebuild
  ```
- Editing the workflow: edit `.claude/skills/reframe-through-structure/SKILL.md`
  (the single source of truth for all three agent hosts), then regenerate the
  Gemini and Copilot variants:
  ```sh
  python tools/gen_agent_prompts.py           # skill → Gemini command + Copilot prompt
  python tools/gen_agent_prompts.py --check   # verify they are current (exit 1 if not)
  ```
- Live edits: open the database file with any SQLite tool; changes take
  effect on the next call (not reflected back into the catalog).
- Reset to seed: delete the database file and restart the server.

## Project layout

```
src/
  main.rs     # thin entry point: CLI, logging → stderr, DB init/seed, transport choice
  http.rs     # streamable HTTP transport (axum): /mcp endpoint, /health, shutdown
  models.rs   # PatternType enum, Pattern, Facets, SearchFilters
  db.rs       # path resolution, schema, seeding, typed queries
  server.rs   # LatServer, the four tools, ServerHandler
  seed.sql    # generated from the catalog (do not edit by hand)
additional_docs/
  lat_catalog.md            # the catalog — single source of truth
  lat_facets.md             # generated snapshot of filter values
                            # (founding essay lives on Zenodo, not in-repo)
tools/
  gen_seed.py               # catalog → src/seed.sql
  gen_facets.py             # catalog → additional_docs/lat_facets.md
  gen_agent_prompts.py      # skill → Gemini command + Copilot prompt
.claude/skills/reframe-through-structure/SKILL.md    # workflow — single source of truth
.github/prompts/reframe-through-structure.prompt.md  # generated — Copilot prompt
.gemini/commands/reframe-through-structure.toml      # generated — Gemini command
```

Logs go to stderr; stdout carries the MCP JSON-RPC stream.

## License

Code (this repository) is licensed under the
[Apache License 2.0](LICENSE). See [NOTICE](NOTICE) for attribution, origin, and
a defensive prior-art disclosure. The Apache-2.0 patent grant (§3) and
warranty/liability disclaimers (§7–8) are the point: clear provenance so
recipients can rely on them.

The pattern catalog — the curated entries in
[`additional_docs/lat_catalog.md`](additional_docs/lat_catalog.md) and the same
data wherever it is carried (the generated `src/seed.sql`, the seeded database) —
is covered by the same Apache-2.0 license as the code, and is not licensed
separately. Reuse it — in other catalogs, datasets or papers — under
Apache-2.0, keeping the attribution and NOTICE requirements of §4. The
grammatical facts it describes are of course not anyone's property; what the
license covers is this curated selection, wording and classification.

The founding essay is a separate work and is not covered by the Apache
license. It is deposited under
[CC BY 4.0](https://creativecommons.org/licenses/by/4.0/) (SPDX: `CC-BY-4.0`) on
Zenodo ([10.5281/zenodo.21382455](https://doi.org/10.5281/zenodo.21382455)) and
PhilArchive ([SCHLAA-18](https://philarchive.org/rec/SCHLAA-18)).

To cite, see [CITATION.cff](CITATION.cff) — it carries `license: Apache-2.0` for
the software — catalog included — and `license: CC-BY-4.0` for the essay
reference. Cite the software under its concept DOI
[10.5281/zenodo.21508868](https://doi.org/10.5281/zenodo.21508868), which covers
all versions; individual releases additionally carry their own version DOI.
 