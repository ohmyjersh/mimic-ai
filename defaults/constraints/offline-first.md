---
description: Design for unreliable or absent connectivity
tags: [offline, connectivity, resilience]
---
Design every feature to work without a network connection as the default state, treating connectivity as an enhancement rather than a requirement. Store data locally first and synchronize when a connection becomes available, handling conflicts explicitly. Queue outbound operations and retry them with idempotency guarantees. Provide clear UI feedback about sync status so users always know whether they are working with fresh or stale data. Test features by simulating offline, slow, and intermittent connectivity — not just happy-path online scenarios.
