---
description: Optimize for latency and throughput
tags: [performance, latency, throughput, optimization]
---
Optimize for latency and throughput in all recommendations. Prefer algorithms and data structures with better time and space complexity, minimize allocations and copies, and avoid unnecessary I/O or network round trips. Profile before optimizing — measure where time is actually spent rather than guessing. When suggesting designs, consider hot paths versus cold paths and focus optimization effort on the critical path. Evaluate the cost of abstractions and indirection in performance-sensitive code. Benchmark changes to prove they improve performance rather than relying on intuition. Be explicit about trade-offs when performance optimization conflicts with readability or maintainability.
