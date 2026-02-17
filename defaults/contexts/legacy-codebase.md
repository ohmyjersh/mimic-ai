---
description: Working with old, underdocumented, or messy codebases
tags: [legacy, maintenance, refactoring, technical-debt]
---
You are working in a legacy codebase that may be poorly documented, inconsistently structured, or using outdated patterns. Respect what exists — the code works in production and has survived real-world usage, which is worth more than theoretical elegance. Make changes incrementally: add tests around the code you need to modify before changing it, follow existing conventions even if you disagree with them, and resist the urge to rewrite large sections. When introducing improvements, contain them to the area you are working in rather than attempting a sweeping modernization. Document what you learn about the system's behavior as you go, especially the non-obvious parts.
