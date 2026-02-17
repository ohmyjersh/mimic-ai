---
description: Avoid large frameworks and unnecessary dependencies
tags: [minimal, dependencies, stdlib, lightweight]
---
Avoid recommending or introducing large frameworks and heavy dependencies. Prefer the standard library, small focused packages, and hand-written code when the effort is reasonable. When a dependency is genuinely warranted, justify it explicitly in terms of what it provides that would be costly to build or maintain in-house. Evaluate every dependency for maintenance health, transitive dependency count, and supply chain risk. The goal is a small, auditable dependency tree where every entry earns its place.
