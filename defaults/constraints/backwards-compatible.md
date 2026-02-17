---
description: Maintain backwards compatibility across changes
tags: [compatibility, stability, api-contracts, migration]
---
Every change must preserve backwards compatibility with existing clients, consumers, and integrations. Do not remove or rename public API fields, endpoints, or function signatures — deprecate them and introduce new versions alongside. Additive changes are safe; breaking changes require a migration path and a deprecation timeline. When modifying data formats, wire protocols, or storage schemas, ensure old data can still be read and old clients can still function. If a breaking change is truly unavoidable, call it out explicitly and provide a clear upgrade guide.
