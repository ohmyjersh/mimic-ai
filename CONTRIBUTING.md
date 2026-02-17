# Contributing to mimic

Thanks for your interest in contributing to mimic. This guide covers how to add new persona fragments and contribute code.

## Adding New Fragments

The easiest way to contribute is by adding new persona, skill, context, tone, or constraint fragments.

### Fragment Format

Each fragment is a markdown file with optional YAML frontmatter:

```markdown
---
description: One-line description shown in the list tool
tags: [relevant, tags]
---
The body text that gets composed into the prompt. Write clear, specific prose
that meaningfully shapes LLM behavior.
```

- `description` is optional (defaults to the first line of the body)
- `tags` is optional (defaults to empty)
- The body should be 2-5 sentences, direct and actionable

### Where to Put Fragments

Add new fragments to the appropriate directory under `defaults/`:

- `defaults/personas/` — core role identities
- `defaults/skills/` — technical expertise areas
- `defaults/contexts/` — situational framing
- `defaults/tones/` — communication styles
- `defaults/constraints/` — rules and boundaries

### Naming Convention

Use lowercase kebab-case for filenames: `my-new-skill.md`

### Submitting

1. Fork the repository
2. Add your fragment file(s) to the appropriate `defaults/` subdirectory
3. Run `cargo test` to make sure everything still passes
4. Open a pull request with a brief description of what you added

> **Note:** PRs that modify files in `defaults/` are automatically scanned for prompt injection using [Llama Prompt Guard 2-86M](https://huggingface.co/meta-llama/Llama-Prompt-Guard-2-86M). If the scanner flags your fragment, review the content to ensure it doesn't contain instructions that could manipulate LLM behavior in unintended ways. You can run the scanner locally with:
>
> ```bash
> pip install -r scripts/requirements-ci.txt
> python scripts/prompt_guard_scanner.py defaults/personas/my-new-fragment.md
> ```

## Contributing Code

### Setup

```bash
git clone https://github.com/<user>/mimic.git
cd mimic
cargo build
cargo test
```

### Code Style

- Run `cargo fmt` before committing
- Run `cargo clippy` and fix any warnings
- Write tests for new functionality
- Keep changes focused — one feature or fix per PR

### Architecture

- `src/fragment.rs` — Fragment struct and markdown parsing
- `src/registry.rs` — Fragment discovery across layers (project, global, built-in)
- `src/compose.rs` — Composition logic
- `src/resolve.rs` — Graph-based fragment discovery (nodes, edges, relations)
- `src/lint.rs` — Fragment validation with 10 lint rules
- `src/server.rs` — MCP server implementation (tools, resources, prompts, completions)
- `src/cli.rs` — CLI argument parsing (lint command)
- `src/lib.rs` — Library crate root (re-exports for integration tests)
- `src/main.rs` — Entry point

### Testing

```bash
cargo test                       # Run all tests (unit + e2e)
cargo clippy -- -D warnings      # Lint (warnings are errors)
cargo fmt -- --check             # Check formatting
cargo run -- lint --warnings     # Validate fragment files
```

The test suite includes:

- **Unit tests** — inline `#[cfg(test)] mod tests` in each module, covering parsing, composition, resolution, and linting logic
- **E2E CLI tests** (`tests/cli.rs`) — exercise the `mimic` binary directly, testing `lint`, `--help`, and `--version` commands, including error exit codes for invalid fragments
- **E2E MCP server tests** (`tests/mcp_server.rs`) — spin up an in-process MCP server and client, testing all three tools (`compose`, `list`, `resolve`), resources, prompts, and project-local fragment overrides

Run `cargo run -- lint --warnings` before submitting fragment PRs to catch validation issues early.
