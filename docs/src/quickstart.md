# Quickstart

Get mimic running in under two minutes.

## 1. Install

```sh
cargo install mimic-ai
```

This installs the `mimic` binary.

## 2. Add to your editor

For Claude Code:

```sh
claude mcp add --transport stdio mimic -- mimic
```

For other editors, see [Editor Setup](editors/claude-code.md).

## 3. Use it

Once connected, your MCP client has access to four tools:

**Get recommendations:**

The `recommend` tool takes a persona name and returns a flat, categorized list of available skills, contexts, tones, and constraints. Use this first to see what fragments are available.

**Compose a prompt:**

The `compose` tool builds a system prompt from fragments. For example, composing a backend engineer with Go and PostgreSQL skills, in a code review context, with a concise tone:

```json
{
  "persona": "backend-engineer",
  "skills": ["go", "postgresql"],
  "contexts": ["code-review"],
  "tones": ["concise"],
  "constraints": ["no-frameworks"]
}
```

This returns a structured system prompt with sections for the persona, expertise, context, communication style, and constraints.

**Browse fragments:**

The `list` tool returns all available personas, skills, contexts, tones, and constraints. You can filter by category, tag, or group.

**Advanced graph discovery:**

The `resolve` tool returns a raw graph of related fragments. Most clients should use `recommend` instead.

## 4. Add project-local fragments (optional)

Create a `.mimic/` directory in your project root to add or override fragments:

```
your-project/
  .mimic/
    personas/
      my-team-lead.md
    skills/
      our-internal-api.md
    constraints/
      company-style-guide.md
```

Project-local fragments take priority over global and built-in defaults. See [Fragments](fragments/overview.md) for details on the file format.

## Built-in fragments

mimic ships with **164 built-in fragments** so you can start immediately:

**Personas (43):** backend-engineer, data-engineer, data-scientist, designer, devops-engineer, engineering-manager, frontend-engineer, fullstack-engineer, ml-engineer, mobile-engineer, platform-engineer, product-manager, qa-engineer, security-engineer, solutions-architect, sre-engineer, technical-writer, plus staff- and principal-level variants of each role

**Skills (80):** agile, ai-ml, android, angular, ansible, api-design, appsec, aws, azure, c, ci-cd, clickhouse, cloud-security, cpp, cryptography, csharp, css, datamodeling, deno, devsecops, docker, dynamodb, elasticsearch, elixir, event-driven, flutter, gcp, git, github-actions, go, graphql, grpc, html, htmx, identity, ios, java, kafka, kotlin, kotlin-mobile, kubernetes, linux, llm, microservices, mongodb, mysql, networking, nextjs, nginx, nix, observability, performance, php, postgresql, python, rabbitmq, react, react-native, redis, regex, rest-api, ruby, rust, scala, serverless, shell-scripting, spark, sql, sqlite, svelte, swift, system-design, tailwind, technical-writing, terraform, testing, typescript, vite, vue, webassembly

**Contexts (18):** architecture-review, code-review, debugging, documentation, estimation, greenfield-project, incident-response, legacy-codebase, mentoring, migration, onboarding, open-source, pair-programming, performance-tuning, post-mortem, prototyping, refactoring, security-audit

**Tones (8):** casual, concise, dry-humor, empathetic, formal, opinionated, pedagogical, socratic

**Constraints (15):** accessibility-first, backwards-compatible, compliance, cost-conscious, deterministic, memory-constrained, minimal-downtime, no-frameworks, offline-first, open-source-only, performance-critical, privacy-first, security-first, test-driven, type-safe
