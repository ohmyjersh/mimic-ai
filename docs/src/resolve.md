# Resolve

The `resolve` tool provides advanced graph-based fragment discovery. Most clients should use the simpler [`recommend`](recommend.md) tool instead, which returns a flat categorized list.

Given a starting persona, tags, or groups, `resolve` returns a graph of fragment nodes connected by edges — useful for debugging relationships or building custom tooling.

## Parameters

| Parameter | Type | Required | Description |
|---|---|---|---|
| `persona` | string | no | Starting persona to anchor the graph (e.g. `"backend-engineer"`) |
| `tags` | string[] | no | Seed tags — only fragments sharing at least one tag are included (tones are always included) |
| `groups` | string[] | no | Filter skills to these groups (overrides the persona's `skill_groups` when provided) |
| `include_edges` | boolean | no | Whether to include edges in the result (default `true`). Set to `false` for a lighter response. |

## Output structure

The result contains three fields:

### `nodes`

An array of fragment nodes. Each node has:

| Field | Type | Description |
|---|---|---|
| `id` | string | Unique identifier: `"{category}:{name}"` (e.g. `"skill:go"`) |
| `category` | string | Fragment category: persona, skill, context, tone, constraint |
| `name` | string | Fragment name |
| `description` | string | Fragment description |
| `tags` | string[] | Associated tags |
| `level` | string? | Seniority level (personas only) |
| `skill_groups` | string[] | Skill groups (personas only) |
| `group` | string? | Group membership (skills only) |
| `source` | string | Source layer: `builtin`, `global`, or `project` |

### `edges`

An array of connections between nodes. Each edge has:

| Field | Type | Description |
|---|---|---|
| `from` | string | Source node ID |
| `to` | string | Target node ID |
| `relation` | string | Edge type: `skill_group`, `group`, or `tag` |
| `label` | string | Human-readable label (the group name or tag) |

Edge types:

- **`skill_group`** -- persona to skill, when the persona's `skill_groups` contains the skill's `group`
- **`group`** -- skill to skill, when they share the same `group`
- **`tag`** -- any to any, when they share an uncommon tag (appearing in 50% or fewer of nodes)

### `meta`

Resolution metadata:

| Field | Type | Description |
|---|---|---|
| `seed` | string? | The persona node ID if one was provided |
| `resolved_groups` | string[] | Effective skill groups used for filtering |
| `resolved_tags` | string[] | User-provided tags |
| `node_count` | number | Total nodes in the result |
| `edge_count` | number | Total edges in the result |

## Use cases

**Discovering related fragments:** Start with a persona and explore which skills, contexts, and constraints are connected to it. The edges show *why* fragments are related.

**Building compose calls from graph walks:** Use `resolve` to find relevant fragments, then pass the discovered names to `compose`. This is especially useful when you don't know the full set of available fragments.

**Filtering by topic:** Pass `tags` like `["security"]` to narrow the graph to security-related fragments across all categories.

**Exploring a skill group:** Pass `groups` like `["frontend"]` to see all frontend-related skills and their connections.

## Example

Resolve with a backend engineer persona:

```json
{
  "persona": "backend-engineer",
  "include_edges": true
}
```

This returns the `backend-engineer` persona node, all skills in its skill groups (backend, data, general), all contexts, tones, and constraints, plus edges showing the relationships between them.
