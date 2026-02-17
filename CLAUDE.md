# CLAUDE.md — mimic-ai

Rust MCP server that composes LLM system prompts from reusable markdown fragments (personas, skills, contexts, tones, constraints).

## Build & Test

```bash
cargo build                      # build
cargo test                       # run all unit tests
cargo test <test_name>           # run a single test
cargo clippy -- -D warnings      # lint (warnings are errors)
cargo fmt -- --check             # check formatting
cargo run -- lint --warnings     # lint fragment files
```

## Architecture

| Module        | Purpose                              |
|---------------|--------------------------------------|
| `fragment.rs` | Parsing markdown fragments (YAML frontmatter + body) |
| `registry.rs` | Fragment discovery and caching       |
| `compose.rs`  | Prompt assembly from selected fragments |
| `resolve.rs`  | Graph-based fragment discovery       |
| `server.rs`   | MCP tool definitions and handlers    |
| `version.rs`  | GitHub version checking and caching  |
| `lint.rs`     | Fragment validation rules            |
| `cli.rs`      | CLI argument parsing                 |
| `main.rs`     | Entry point                          |

**Three-layer fragment resolution** (highest priority wins):
1. Project-local `.mimic/` directory (highest)
2. Global `~/.mimic/` directory
3. Built-in defaults compiled into the binary (lowest)

**Fragment categories:** Persona, Skill, Context, Tone, Constraint

## Code Conventions

- **Error handling:** `Result<T, String>` everywhere — no custom error types. Use `format!()` for descriptive error messages.
- **Module organization:** flat in `src/`, one file per module, no nested submodules.
- **Async:** single-threaded Tokio (`flavor = "current_thread"`).
- **Logging:** `eprintln!("mimic: ...")` to stderr — no logging framework.
- **Derives:** `#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]` on data types; `#[serde(rename_all = "lowercase")]` on enums.
- **Dependencies:** use crate APIs directly, no wrapper abstractions.
- **No `unsafe` code.**
- **Visibility:** `pub` for cross-module items, private helpers stay private.

## TDD Rules

- **Strict test-first (red-green-refactor):** write a failing test that defines expected behavior, make it pass with minimal code, then refactor. Do not skip the red step.
- **Test location:** inline `#[cfg(test)] mod tests { use super::*; }` at the bottom of each module — no separate test files.
- **Test naming:** descriptive snake_case describing what's tested and the expected outcome (e.g., `project_local_overrides_builtin`, `valid_yaml_fails_on_bad_yaml`).
- **No mocking frameworks** — design for testability via function parameters and flexible constructors (e.g., `Registry::new(Option<PathBuf>)`).
- **Filesystem tests:** use `tempfile::tempdir()` for isolated filesystem operations.
- **Test helpers:** keep minimal — small helpers like `fn registry() -> Registry` or `fn make_ctx(...)` at the top of the test module.
- **Coverage:** test critical paths, boundary conditions, and error cases. Every `Result::Err` path should have a test.
- **Test structure:** Arrange-Act-Assert pattern. No test should depend on another test's state.
- **Run `cargo test` before every commit** — CI enforces this.

## Dependency Policy

- Be conservative — prefer `std` and existing crates over adding new dependencies.
- Discuss and justify before adding any new crate to `Cargo.toml`.
- Keep direct dependency count minimal (~11 crates currently).
- Use crate APIs directly, no wrapper abstractions.

## Fragment Conventions

- **Filename:** lowercase kebab-case `.md`
- **Frontmatter fields:** `description`, `tags`, `group`, `level`, `skill_groups`, `category` (root-level only)
- **Body:** 2-5 sentences, direct and actionable prose

## CI Checks (all must pass)

```bash
cargo test --all-features
cargo clippy -- -D warnings
cargo fmt -- --check
cargo run -- lint --warnings
cd extensions/vscode && npm ci && npm run compile   # VS Code extension
```
