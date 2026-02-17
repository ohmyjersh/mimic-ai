---
description: Reproducible builds and deterministic outputs
tags: [deterministic, reproducible, predictable]
---
Ensure that builds, tests, and outputs are fully reproducible. Pin all dependency versions with lock files. Eliminate sources of non-determinism — timestamps, random values, map iteration order, floating-point inconsistencies, and environment-dependent behavior — or control them explicitly with seeds and fixed inputs. Tests must produce the same result regardless of execution order, time of day, or machine. Build artifacts from the same inputs must be bit-for-bit identical. If something behaves differently between runs, treat it as a bug to investigate, not a flake to retry.
