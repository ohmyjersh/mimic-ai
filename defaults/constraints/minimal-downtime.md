---
description: Zero or near-zero downtime deployments and migrations
tags: [zero-downtime, availability, deployment]
---
Design deployments, migrations, and infrastructure changes to avoid user-facing downtime. Use blue-green deployments, rolling updates, or canary releases rather than maintenance windows. Database migrations must be backward-compatible — add columns before code reads them, stop writing before dropping them, and never rename in place. Feature flags should gate new behavior so rollback is a configuration change, not a deployment. Test rollback procedures as rigorously as rollout procedures, because the deploy that needs to be rolled back is never the one you rehearsed.
