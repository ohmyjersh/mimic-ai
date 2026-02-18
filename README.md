# mimic

Composable persona prompts for any LLM — an MCP server.

mimic constructs structured LLM system prompts from reusable markdown fragments. Drop `.md` files into categorized directories, and mimic composes them into complete personas. Works across all MCP-compatible clients: Claude Code, Codex CLI, Gemini CLI, Cursor, VS Code, Windsurf, Zed, and more.

Ships with sensible defaults — plug and play out of the box.

## Quick Start

Install:

```bash
cargo install mimic-ai
```

Add to Claude Code:

```bash
claude mcp add --transport stdio mimic -- mimic
```

Use:

```
> compose a backend engineer persona with go and postgresql skills
```

mimic's `compose` tool returns:

```
You are a senior backend engineer with deep experience building distributed systems,
designing APIs, and operating production services at scale...

## Expertise

You have expert-level proficiency in Go...

You have deep expertise in PostgreSQL...
```

## How It Works

mimic organizes prompt fragments into five categories:

| Category | Purpose | Example |
|---|---|---|
| **Persona** | Core role identity | `backend-engineer`, `technical-writer` |
| **Skill** | Technical expertise | `go`, `postgresql`, `kubernetes` |
| **Context** | Situational framing | `code-review`, `debugging` |
| **Tone** | Communication style | `concise`, `pedagogical` |
| **Constraint** | Rules and boundaries | `no-frameworks`, `security-first` |

Fragments compose in fixed order: persona + skills + contexts + tones + constraints.

### Fragment Format

Minimal markdown with optional YAML frontmatter:

```markdown
---
description: Senior backend engineer focused on distributed systems
tags: [backend, distributed-systems, apis]
---
You are a senior backend engineer with deep experience building distributed systems,
designing APIs, and operating production services at scale.
```

### Layered Resolution

Fragments resolve in priority order:

1. **Project-local** (`.mimic/`) — project-specific overrides
2. **User global** (`~/.mimic/`) — your personal defaults
3. **Built-in** — ships with the binary

Override any built-in fragment by placing a file with the same name in your global or project-local directory.

## MCP Interface

### Tools

**`recommend`** — Get recommended fragments for a persona. Returns a flat, categorized list of skills, contexts, tones, and constraints that match the persona's skill groups. Use this before `compose` to see what's available.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `persona` | string | yes | Persona name (e.g. `"backend-engineer"`) |
| `groups` | string[] | no | Override the persona's skill groups (e.g. `["backend", "data"]`) |
| `tags` | string[] | no | Filter recommendations by tags (e.g. `["security"]`) |

**`compose`** — Build a system prompt from fragments.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `persona` | string | yes | Persona name (e.g. `"backend-engineer"`) |
| `skills` | string[] | no | Skill names (e.g. `["go", "postgresql"]`) |
| `contexts` | string[] | no | Context names (e.g. `["code-review", "greenfield-project"]`) |
| `tones` | string[] | no | Tone names (e.g. `["concise", "pedagogical"]`) |
| `constraints` | string[] | no | Constraint names (e.g. `["no-frameworks"]`) |

**`list`** — Browse available fragments. Returns JSON with name, category, description, and tags. Optionally filter by `category`, `tag`, or `group`.

**`check_update`** — Check if a newer version of mimic is available. Returns JSON with `current`, `latest`, and `update_available` fields. Results are cached for 1 hour.

> **Note:** When `check_update` has been called and an update is available, `compose` will automatically append an update notice to its output.

**`resolve`** — Advanced: returns a raw graph of fragment relationships. Most clients should use `recommend` instead. Given a persona, tags, or groups, returns nodes and edges connected by shared tags, groups, and skill groups.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `persona` | string | no | Starting persona to anchor the graph |
| `tags` | string[] | no | Seed tags — only fragments sharing at least one tag are included (tones always included) |
| `groups` | string[] | no | Filter skills to these groups (overrides persona's skill_groups) |
| `include_edges` | boolean | no | Include edges in the result (default `true`) |

### Resources

Browse fragments individually via `mimic://fragments/{category}/{name}` URIs.

### Prompts

Common compositions exposed as MCP prompts (e.g. `/mimic-backend-engineer`).

### Completions

Autocomplete support for all tool parameters — any client that supports MCP completions gets typeahead for free.

## Editor Setup

### Claude Code

```bash
claude mcp add --transport stdio mimic -- mimic
```

### Cursor / VS Code

Add to `.cursor/mcp.json` or VS Code MCP settings:

```json
{
  "mcpServers": {
    "mimic": {
      "command": "mimic",
      "args": [],
      "transportType": "stdio"
    }
  }
}
```

### Codex CLI

```bash
codex mcp add mimic -- mimic
```

See the [full documentation](https://github.com/ohmyjersh/mimic-ai/tree/main/docs) for more editor setup guides.

## Custom Fragments

Create your own fragments in `~/.mimic/` (global) or `.mimic/` (project-local):

```bash
mkdir -p ~/.mimic/skills
cat > ~/.mimic/skills/elixir.md << 'EOF'
---
description: Elixir and OTP expertise
tags: [language, elixir, beam]
---
You have deep expertise in Elixir and the OTP framework. You write idiomatic
Elixir using GenServers, Supervisors, and the actor model. You understand BEAM
VM characteristics and can reason about fault tolerance and concurrency.
EOF
```

## Linting

Validate your fragment files for correctness:

```bash
mimic lint              # show errors only
mimic lint --warnings   # show errors and warnings
```

The linter checks built-in, global (`~/.mimic/`), and project-local (`.mimic/`) fragments. Exit code is 1 if any errors are found.

## Built-in Fragments

mimic ships with **164 built-in fragments** across all five categories.

**Personas (43):** backend-engineer, data-engineer, data-scientist, designer, devops-engineer, engineering-manager, frontend-engineer, fullstack-engineer, ml-engineer, mobile-engineer, platform-engineer, product-manager, qa-engineer, security-engineer, solutions-architect, sre-engineer, technical-writer, staff-backend-engineer, staff-data-engineer, staff-designer, staff-devops-engineer, staff-engineering-manager, staff-frontend-engineer, staff-ml-engineer, staff-mobile-engineer, staff-platform-engineer, staff-product-manager, staff-qa-engineer, staff-security-engineer, staff-technical-writer, principal-backend-engineer, principal-data-engineer, principal-designer, principal-devops-engineer, principal-engineering-manager, principal-frontend-engineer, principal-ml-engineer, principal-mobile-engineer, principal-platform-engineer, principal-product-manager, principal-qa-engineer, principal-security-engineer, principal-technical-writer

**Skills (80):** agile, ai-ml, android, angular, ansible, api-design, appsec, aws, azure, c, ci-cd, clickhouse, cloud-security, cpp, cryptography, csharp, css, datamodeling, deno, devsecops, docker, dynamodb, elasticsearch, elixir, event-driven, flutter, gcp, git, github-actions, go, graphql, grpc, html, htmx, identity, ios, java, kafka, kotlin, kotlin-mobile, kubernetes, linux, llm, microservices, mongodb, mysql, networking, nextjs, nginx, nix, observability, performance, php, postgresql, python, rabbitmq, react, react-native, redis, regex, rest-api, ruby, rust, scala, serverless, shell-scripting, spark, sql, sqlite, svelte, swift, system-design, tailwind, technical-writing, terraform, testing, typescript, vite, vue, webassembly

**Contexts (18):** architecture-review, code-review, debugging, documentation, estimation, greenfield-project, incident-response, legacy-codebase, mentoring, migration, onboarding, open-source, pair-programming, performance-tuning, post-mortem, prototyping, refactoring, security-audit

**Tones (8):** casual, concise, dry-humor, empathetic, formal, opinionated, pedagogical, socratic

**Constraints (15):** accessibility-first, backwards-compatible, compliance, cost-conscious, deterministic, memory-constrained, minimal-downtime, no-frameworks, offline-first, open-source-only, performance-critical, privacy-first, security-first, test-driven, type-safe

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md). The easiest way to contribute is adding new fragment files — no Rust knowledge required.

## License

MIT
