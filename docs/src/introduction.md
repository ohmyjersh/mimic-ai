# Introduction

**mimic** is an MCP server that composes LLM persona prompts from reusable markdown files. It works across all MCP-compatible clients -- Claude Code, Cursor, VS Code, Codex CLI, Gemini CLI, and others.

## The problem

When using LLM-powered coding assistants, the system prompt determines how the model behaves. Most developers either use the default prompt (generic) or write one-off instructions (hard to reuse). There is no standard way to say "act as a backend engineer who knows Go and PostgreSQL, reviewing code, in a concise tone, without recommending frameworks."

## How mimic solves it

mimic breaks persona prompts into five composable categories:

- **Persona** -- who the model is (e.g. backend engineer, technical writer)
- **Skill** -- what technologies it knows (e.g. Rust, Kubernetes, React)
- **Context** -- what task it is doing (e.g. code review, debugging)
- **Tone** -- how it communicates (e.g. concise, pedagogical)
- **Constraint** -- what rules it must follow (e.g. no frameworks, security first)

Each fragment is a standalone markdown file. mimic composes them into a single structured system prompt at runtime. Fragments can be shared across teams, checked into repos, or customized per project.

## How it works

mimic exposes five MCP tools:

- `recommend` -- takes a persona name and returns a flat, categorized list of available skills, contexts, tones, and constraints. Use this first to see what's available.
- `compose` -- takes a persona plus optional skills, contexts, tones, and constraints, and returns a complete system prompt.
- `list` -- returns all available fragments, optionally filtered by category, tag, or group.
- `check_update` -- checks if a newer version of mimic is available. Returns current version, latest version, and whether an update is available.
- `resolve` -- advanced: returns a raw graph of fragment relationships. Most clients should use `recommend` instead.

The MCP client (your editor) calls these tools. The LLM decides which fragments to use based on your conversation, or you can specify them explicitly.

## Design principles

- **Composable** -- small, single-purpose fragments that combine cleanly.
- **Layered** -- project-local fragments override global ones, which override built-in defaults.
- **Plain text** -- every fragment is a markdown file with optional YAML frontmatter. No databases, no config formats to learn.
- **Client-agnostic** -- works with any MCP-compatible tool over stdio transport.
