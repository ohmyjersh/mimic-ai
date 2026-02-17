# Fragments

Fragments are the building blocks of mimic. Each fragment is a markdown file that contributes one piece of a composed system prompt.

## Categories

mimic organizes fragments into five categories:

| Category | Directory | Purpose | Section header |
|---|---|---|---|
| Persona | `personas/` | Who the model is | *(none -- appears first)* |
| Skill | `skills/` | Technology expertise | `## Expertise` |
| Context | `contexts/` | What task is being done | `## Context` |
| Tone | `tones/` | Communication style | `## Communication Style` |
| Constraint | `constraints/` | Rules and restrictions | `## Constraints` |

When composed, fragments are assembled in this order: persona body first, then skills under an "Expertise" heading, context, tone, and constraints -- each under their own heading.

## Layering

mimic loads fragments from three sources, in order of increasing priority:

1. **Built-in** -- shipped inside the mimic binary. These are the defaults.
2. **Global** -- loaded from `~/.mimic/`. Useful for personal preferences that apply across all projects.
3. **Project-local** -- loaded from `.mimic/` in the project root (found by walking up from the current directory). Useful for team or project-specific fragments.

If two fragments share the same category and filename, the higher-priority source wins. For example, a `.mimic/personas/backend-engineer.md` in your project completely replaces the built-in `backend-engineer` persona.

## File format

Each fragment is a markdown file (`.md`) with optional YAML frontmatter:

```markdown
---
description: Senior backend engineer focused on distributed systems
tags: [backend, distributed-systems, apis]
---
You are a senior backend engineer with deep experience building
and operating distributed systems at scale.
```

### Frontmatter fields

All fields are optional:

- **description** -- a short summary shown in `list` results. If omitted, mimic uses the first line of the body.
- **tags** -- an array of strings used for filtering with the `list` and `resolve` tools.
- **group** -- the skill group this fragment belongs to (skills only). Used by `resolve` to connect skills to personas. Values: `backend`, `frontend`, `mobile`, `infrastructure`, `data`, `security`, `general`.
- **level** -- seniority level (personas only). Values: `senior`, `staff`, `principal`.
- **skill_groups** -- which skill groups this persona is associated with (personas only). Controls which skills `resolve` links to this persona.
- **category** -- the fragment category (root-level files only). Required when a fragment file is placed directly in the `.mimic/` or `~/.mimic/` root instead of a category subdirectory. Values: `persona`, `skill`, `context`, `tone`, `constraint`.

### Body

The body is the actual prompt text that gets included in the composed output. Write it in second person ("You are...", "You write...") for personas and skills, or as direct instructions for tones and constraints.

## Directory structure

A complete setup with all three layers might look like this:

```
# Built-in (compiled into the binary)
defaults/
  personas/backend-engineer.md
  skills/rust.md
  tones/concise.md
  ...

# Global (~/.mimic/)
~/.mimic
  personas/my-custom-persona.md
  tones/casual.md

# Project-local (.mimic/ in repo root)
your-project/.mimic/
  personas/backend-engineer.md   # overrides built-in
  skills/internal-api.md         # project-specific addition
  constraints/house-style.md
```

## Naming

Fragment names are derived from the filename without the `.md` extension. Use lowercase kebab-case: `backend-engineer.md`, `code-review.md`, `security-first.md`. The name is what you pass to the `compose` tool.
