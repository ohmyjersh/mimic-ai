# Recommend

The `recommend` tool returns a flat, categorized list of recommended fragments for a given persona. It is the simplest way to discover what skills, contexts, tones, and constraints are available before calling `compose`.

## Parameters

| Parameter | Type | Required | Description |
|---|---|---|---|
| `persona` | string | yes | The persona to get recommendations for (e.g. `"backend-engineer"`) |
| `groups` | string[] | no | Override the persona's skill groups (e.g. `["backend", "data"]`). Empty = use persona's groups. |
| `tags` | string[] | no | Filter recommendations by tags (e.g. `["security"]`). Empty = no tag filter. |

## Output structure

The result is a JSON object with five fields:

### `persona`

Information about the requested persona:

| Field | Type | Description |
|---|---|---|
| `name` | string | Persona name |
| `description` | string | Persona description |
| `level` | string? | Seniority level |
| `skill_groups` | string[] | Persona's skill groups |

### `skills`

Array of recommended skills, filtered by the persona's skill groups (or the `groups` override):

| Field | Type | Description |
|---|---|---|
| `name` | string | Skill name |
| `description` | string | Skill description |
| `group` | string? | Skill group membership |

### `contexts`

Array of available contexts (filtered by `tags` if provided).

### `tones`

Array of all available tones (always included, not filtered by tags).

### `constraints`

Array of available constraints (filtered by `tags` if provided).

## Example

Get recommendations for a backend engineer:

```json
{
  "persona": "backend-engineer"
}
```

Override skill groups to see frontend skills:

```json
{
  "persona": "backend-engineer",
  "groups": ["frontend"]
}
```

Filter to security-related fragments:

```json
{
  "persona": "backend-engineer",
  "tags": ["security"]
}
```

## Workflow

The recommended workflow is:

1. Call `recommend` with a persona name to see available fragments.
2. Pick the skills, contexts, tones, and constraints you want.
3. Call `compose` with those fragment names to build a system prompt.

For advanced graph-based discovery, see [Resolve](resolve.md).
