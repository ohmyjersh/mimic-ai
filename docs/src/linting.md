# Linting

mimic includes a built-in linter that validates fragment files for correctness. It checks built-in defaults, global fragments (`~/.mimic/`), and project-local fragments (`.mimic/`).

## Usage

```bash
mimic lint              # show errors only
mimic lint --warnings   # show errors and warnings
```

The linter exits with code `1` if any errors are found, `0` otherwise. Warnings do not affect the exit code.

## Rules

The linter runs 10 rules, each producing either an error or a warning:

### Errors

| Rule | Description |
|---|---|
| `valid-yaml` | YAML frontmatter must be syntactically valid |
| `non-empty-body` | Fragment body must not be empty or whitespace-only |
| `root-file-has-category` | Root-level files (not in a category subdirectory) must have a `category` field in frontmatter with a valid value |

### Warnings

| Rule | Description |
|---|---|
| `has-description` | Fragment should have a `description` field in frontmatter |
| `has-tags` | Fragment should have a non-empty `tags` array in frontmatter |
| `unknown-fields` | Frontmatter should not contain unrecognized field names |
| `skill-has-group` | Skill fragments should have a `group` field |
| `persona-has-level` | Persona fragments should have a `level` field |
| `persona-has-skill-groups` | Persona fragments should have a `skill_groups` field |
| `category-conflict` | Frontmatter `category` should match the subdirectory the file is in |

## Output format

Diagnostics are printed to stderr:

```
  error [valid-yaml] .mimic/skills/bad.md: invalid YAML frontmatter: ...
  warning [has-description] .mimic/skills/my-skill.md: missing `description` field in frontmatter
```

When no issues are found:

```
All fragments OK.
```

When only warnings are found (without `--warnings`):

```
All fragments OK (3 warning(s) hidden, use --warnings to show).
```
