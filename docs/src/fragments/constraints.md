# Constraints

A constraint defines *rules* the model must follow. Constraints appear under the `## Constraints` heading in the composed output. You can include multiple constraints in a single compose request.

## Writing a constraint

A good constraint is specific and actionable. It tells the model what to do (or not do) and why.

### Example

```markdown
---
description: Avoid large frameworks and unnecessary dependencies
tags: [minimal, dependencies, stdlib, lightweight]
---
Avoid recommending or introducing large frameworks and heavy dependencies.
Prefer the standard library, small focused packages, and hand-written code
when the effort is reasonable. When a dependency is genuinely warranted,
justify it explicitly in terms of what it provides that would be costly to
build or maintain in-house. Evaluate every dependency for maintenance health,
transitive dependency count, and supply chain risk. The goal is a small,
auditable dependency tree where every entry earns its place.
```

### Guidelines

- Write as direct instructions: "Avoid...", "Prefer...", "Always..."
- State the constraint and the reasoning behind it.
- Be specific enough that the model can follow the rule without ambiguity.
- Constraints stack. Write each one to be independent of the others.

## Built-in constraints

| Name | Description |
|---|---|
| `accessibility-first` | WCAG compliance, inclusive design |
| `backwards-compatible` | Maintain backwards compatibility across changes |
| `compliance` | Regulatory compliance (SOC2, HIPAA, PCI-DSS) |
| `cost-conscious` | Minimize cloud spend and operational costs |
| `deterministic` | Reproducible builds and deterministic outputs |
| `memory-constrained` | Design for limited memory environments |
| `minimal-downtime` | Zero or near-zero downtime deployments and migrations |
| `no-frameworks` | Avoid large frameworks and unnecessary dependencies |
| `offline-first` | Design for unreliable or absent connectivity |
| `open-source-only` | Use only open-source dependencies |
| `performance-critical` | Optimize for latency and throughput |
| `privacy-first` | GDPR/CCPA compliance, data minimization |
| `security-first` | Prioritize security in all recommendations |
| `test-driven` | Write tests before implementation |
| `type-safe` | Maximize type safety and static analysis |

## Composing multiple constraints

Pass an array of constraint names:

```json
{
  "persona": "backend-engineer",
  "skills": ["rust"],
  "constraints": ["security-first", "no-frameworks"]
}
```

All constraint bodies are concatenated under a single `## Constraints` heading, separated by blank lines.
