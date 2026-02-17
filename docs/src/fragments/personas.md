# Personas

A persona defines *who* the model is. It is the only required argument to the `compose` tool and always appears first in the output, before any section headings.

## Writing a persona

A good persona establishes identity, priorities, and judgment. Focus on how this person *thinks* and *makes decisions*, not just what they know.

### Example

```markdown
---
description: Senior backend engineer focused on distributed systems, APIs, and reliability
tags: [backend, distributed-systems, apis, reliability]
---
You are a senior backend engineer with deep experience building and operating
distributed systems at scale. You think in terms of data flow, failure modes,
and system boundaries before reaching for specific technologies. When designing
APIs, you favor clear contracts, backward compatibility, and sensible defaults
over cleverness. You treat observability, graceful degradation, and operational
simplicity as first-class requirements rather than afterthoughts. Your code
reviews focus on correctness under concurrency, error handling edge cases, and
whether the abstraction will survive the next requirement change.
```

### Guidelines

- Write in second person: "You are...", "You think...", "You favor..."
- State values and trade-off preferences, not just capabilities.
- Keep it to one paragraph. The persona sets the frame; skills, context, and tone add the specifics.
- Avoid listing technologies -- that is what skills are for.

## Built-in personas

### Senior level

| Name | Description |
|---|---|
| `backend-engineer` | Senior backend engineer focused on distributed systems, APIs, and reliability |
| `data-engineer` | Senior data engineer focused on pipelines, warehousing, and data reliability |
| `data-scientist` | Senior data scientist focused on statistical analysis, experimentation, and modeling |
| `designer` | Designer focused on user experience, interface design, and research-driven decisions |
| `devops-engineer` | Senior DevOps/SRE focused on infrastructure, CI/CD, and observability |
| `engineering-manager` | Engineering manager focused on team health, delivery, and growing engineers |
| `frontend-engineer` | Senior frontend engineer focused on UI/UX, accessibility, and performance |
| `fullstack-engineer` | Senior fullstack engineer bridging frontend and backend |
| `ml-engineer` | Senior ML engineer focused on building, deploying, and operating machine learning systems |
| `mobile-engineer` | Senior mobile engineer focused on iOS, Android, and cross-platform development |
| `platform-engineer` | Senior platform engineer focused on internal developer platforms, tooling, and developer experience |
| `product-manager` | Product manager focused on strategy, discovery, and stakeholder alignment |
| `qa-engineer` | QA engineer focused on test strategy, automation, and quality advocacy |
| `security-engineer` | Senior security engineer focused on application security, infrastructure hardening, and compliance |
| `solutions-architect` | Senior solutions architect bridging business requirements and technical architecture |
| `sre-engineer` | Senior SRE focused on reliability, SLOs, error budgets, and toil reduction |
| `technical-writer` | Technical writer focused on clear, accurate documentation |

### Staff level

| Name | Description |
|---|---|
| `staff-backend-engineer` | Staff backend engineer driving cross-team architecture, system design, and backend standards |
| `staff-data-engineer` | Staff data engineer driving cross-team data architecture, platform standards, and data governance |
| `staff-designer` | Design Lead driving design systems, standards, and cross-team design strategy |
| `staff-devops-engineer` | Staff DevOps engineer driving platform standards, infrastructure architecture, and cross-team reliability |
| `staff-engineering-manager` | Director of Engineering managing multiple teams, delivery, and engineering strategy |
| `staff-frontend-engineer` | Staff frontend engineer driving design systems, frontend architecture, and cross-team UI standards |
| `staff-ml-engineer` | Staff ML engineer driving cross-team ML platform, model standards, and production ML practices |
| `staff-mobile-engineer` | Staff mobile engineer driving cross-team mobile architecture, shared frameworks, and platform standards |
| `staff-platform-engineer` | Staff platform engineer driving cross-team platform architecture, developer experience strategy, and internal tooling standards |
| `staff-product-manager` | Group Product Manager leading product strategy across multiple teams |
| `staff-qa-engineer` | QA Lead/Architect designing org-wide test strategy and quality infrastructure |
| `staff-security-engineer` | Staff security engineer driving cross-team security architecture, standards, and threat modeling practices |
| `staff-technical-writer` | Documentation Lead driving content strategy and documentation standards |

### Principal level

| Name | Description |
|---|---|
| `principal-backend-engineer` | Principal backend engineer setting org-wide technical direction and multi-year backend strategy |
| `principal-data-engineer` | Principal data engineer defining org-wide data strategy, governance, and multi-year platform vision |
| `principal-designer` | Head of Design defining company-wide design vision and culture |
| `principal-devops-engineer` | Principal DevOps engineer setting org-wide infrastructure strategy and technical direction for reliability |
| `principal-engineering-manager` | VP of Engineering setting org-wide engineering vision and culture |
| `principal-frontend-engineer` | Principal frontend engineer defining org-wide frontend platform strategy and technical direction |
| `principal-ml-engineer` | Principal ML engineer setting org-wide ML strategy, responsible AI practices, and multi-year platform direction |
| `principal-mobile-engineer` | Principal mobile engineer setting org-wide mobile strategy and multi-year platform direction |
| `principal-platform-engineer` | Principal platform engineer defining org-wide developer platform vision and multi-year infrastructure strategy |
| `principal-product-manager` | Head of Product defining company-wide product vision and strategy |
| `principal-qa-engineer` | Head of Quality defining org-wide quality vision and engineering excellence |
| `principal-security-engineer` | Principal security engineer setting org-wide security strategy, risk framework, and multi-year security roadmap |
| `principal-technical-writer` | Head of Documentation defining content strategy and developer experience |

## Overriding a built-in persona

Create a file with the same name in your `.mimic/personas/` directory. Your version completely replaces the built-in one:

```
.mimic/
  personas/
    backend-engineer.md   # your version replaces the default
```
