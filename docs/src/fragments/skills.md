# Skills

A skill describes expertise in a specific technology, language, or tool. Skills appear under the `## Expertise` heading in the composed output. You can include multiple skills in a single compose request.

## Writing a skill

A good skill fragment describes *how* the model uses a technology, not just that it knows it. Focus on idioms, best practices, and the trade-offs that distinguish an expert from a beginner.

### Example

```markdown
---
description: Rust programming language expertise
tags: [rust, systems-programming, memory-safety]
---
You write idiomatic Rust that leverages the type system and ownership model
to eliminate entire classes of bugs at compile time. You reach for enums and
pattern matching to make invalid states unrepresentable, use traits to define
behavior boundaries, and understand when to use generics versus trait objects.
You are fluent in lifetime annotations, the borrow checker's reasoning, and
common patterns for interior mutability. You write code that handles errors
with the Result type and the ? operator rather than panicking, and you know
when to use thiserror versus anyhow. You are pragmatic about unsafe -- you
understand when it is necessary and how to encapsulate it safely.
```

### Guidelines

- Write in second person: "You write...", "You reach for...", "You understand..."
- Describe expert-level usage, not introductory knowledge.
- Include the idioms and patterns that define good usage of the technology.
- Keep each skill self-contained. It should make sense combined with any persona.

## Built-in skills

| Name | Description |
|---|---|
| `agile` | Agile practices, sprint planning, and team process expertise |
| `ai-ml` | Machine learning fundamentals, model training, and MLOps expertise |
| `android` | Android platform expertise — lifecycle, permissions, Material Design |
| `angular` | Angular framework expertise with RxJS and dependency injection |
| `ansible` | Ansible playbooks, roles, and infrastructure automation expertise |
| `api-design` | API design principles across REST, GraphQL, gRPC, and beyond |
| `appsec` | Application security expertise covering OWASP Top 10 and secure coding practices |
| `aws` | AWS cloud services expertise |
| `azure` | Microsoft Azure services and architecture expertise |
| `c` | C programming language and systems-level development expertise |
| `ci-cd` | CI/CD pipeline design, automation, and deployment strategies |
| `clickhouse` | ClickHouse columnar database, query optimization, and analytics workloads |
| `cloud-security` | Cloud security posture management, IAM policies, and network security |
| `cpp` | Modern C++ programming expertise (C++17/20/23) |
| `cryptography` | Cryptographic primitives, TLS, and key management expertise |
| `csharp` | C# programming language and .NET runtime expertise |
| `css` | CSS architecture, layout systems, and responsive design |
| `datamodeling` | Data modeling principles, normalization, and dimensional modeling expertise |
| `deno` | Deno runtime, permissions model, and TypeScript-first development |
| `devsecops` | Security integration in CI/CD pipelines, SAST/DAST, and supply chain security |
| `docker` | Docker containerization expertise |
| `dynamodb` | DynamoDB data modeling, query design, and operational expertise |
| `elasticsearch` | Elasticsearch search engine and analytics expertise |
| `elixir` | Elixir programming language and OTP platform expertise |
| `event-driven` | Event-driven architecture, CQRS, event sourcing, and messaging patterns |
| `flutter` | Flutter and Dart expertise with widget architecture |
| `gcp` | Google Cloud Platform services and architecture expertise |
| `git` | Git version control expertise |
| `github-actions` | GitHub Actions workflows and CI/CD pipeline expertise |
| `go` | Go programming language expertise |
| `graphql` | GraphQL API design, schema architecture, and federation expertise |
| `grpc` | gRPC and Protocol Buffers expertise for service-to-service communication |
| `html` | Semantic HTML, accessibility, and web standards |
| `htmx` | htmx hypermedia-driven web development |
| `identity` | Authentication and authorization expertise with OAuth2, OIDC, and RBAC |
| `ios` | iOS platform expertise — App Store, Core Data, push notifications |
| `java` | Java programming language and JVM platform expertise |
| `kafka` | Apache Kafka event streaming platform expertise |
| `kotlin` | Kotlin programming language and server-side development expertise |
| `kotlin-mobile` | Kotlin for Android with Jetpack Compose and Android SDK |
| `kubernetes` | Kubernetes orchestration expertise |
| `linux` | Linux administration, shell scripting, and system internals expertise |
| `llm` | LLM integration, prompt engineering, RAG, and embeddings expertise |
| `microservices` | Microservices architecture, service decomposition, and distributed system design |
| `mongodb` | MongoDB document database design and operations expertise |
| `mysql` | MySQL and MariaDB database administration and optimization expertise |
| `networking` | Networking fundamentals including TCP/IP, DNS, HTTP, TLS, and load balancing |
| `nextjs` | Next.js expertise with App Router and React Server Components |
| `nginx` | Nginx configuration, reverse proxy, and load balancing expertise |
| `nix` | Nix package manager, NixOS, and reproducible environments |
| `observability` | Monitoring, logging, and tracing with Prometheus, Grafana, and OpenTelemetry |
| `performance` | Performance optimization, profiling, and benchmarking expertise |
| `php` | Modern PHP programming and Laravel framework expertise |
| `postgresql` | PostgreSQL database expertise |
| `python` | Python programming expertise |
| `rabbitmq` | RabbitMQ messaging, queue design, and AMQP expertise |
| `react` | React framework expertise |
| `react-native` | React Native cross-platform mobile development |
| `redis` | Redis data structures, caching patterns, and messaging expertise |
| `regex` | Regular expression design, optimization, and debugging |
| `rest-api` | REST API design, HTTP semantics, and versioning expertise |
| `ruby` | Ruby programming language and Rails framework expertise |
| `rust` | Rust programming language expertise |
| `scala` | Scala programming language expertise with functional and OOP patterns |
| `serverless` | Serverless architecture, functions-as-a-service, and event-driven compute |
| `shell-scripting` | Bash and shell scripting for automation and tooling |
| `spark` | Apache Spark distributed data processing expertise |
| `sql` | SQL fundamentals, query optimization, and schema design expertise |
| `sqlite` | SQLite embedded database usage, optimization, and best practices |
| `svelte` | Svelte and SvelteKit expertise with compile-time reactivity |
| `swift` | Swift language expertise with SwiftUI and UIKit |
| `system-design` | Distributed system design, scalability patterns, and architectural trade-offs |
| `tailwind` | Tailwind CSS utility-first patterns and design systems |
| `technical-writing` | Technical documentation, RFCs, and ADR writing expertise |
| `terraform` | Terraform infrastructure-as-code expertise with state management and module design |
| `testing` | Testing strategy, unit/integration/e2e testing, and TDD patterns |
| `typescript` | TypeScript and JavaScript expertise |
| `vite` | Vite build tooling, configuration, and plugin development |
| `vue` | Vue 3 framework expertise with Composition API and reactivity |
| `webassembly` | WebAssembly, Rust/C++ to WASM compilation, and browser integration |

## Composing multiple skills

Pass an array of skill names to include more than one:

```json
{
  "persona": "backend-engineer",
  "skills": ["go", "postgresql", "docker"]
}
```

All skill bodies are concatenated under a single `## Expertise` heading, separated by blank lines.
