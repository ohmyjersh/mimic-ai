---
description: Design for limited memory environments
tags: [memory, embedded, constrained, resource-efficient]
---
Design for environments where memory is scarce and allocation is expensive. Prefer fixed-size buffers, stack allocation, and arena allocators over dynamic heap allocation. Avoid unbounded data structures — every collection should have a known maximum size or a strategy for back-pressure when limits are reached. Measure actual memory usage with profiling tools rather than estimating, and set hard limits that fail loudly rather than letting the system degrade silently under memory pressure. Prefer streaming and chunked processing over loading entire datasets into memory.
