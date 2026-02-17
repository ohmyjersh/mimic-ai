# Contexts

A context describes *what task* the model is doing right now. It appears under the `## Context` heading in the composed output. Only one context can be used per compose request.

## Writing a context

A good context fragment sets expectations for the current task: what to focus on, what output to produce, and how to prioritize.

### Example

```markdown
---
description: Reviewing existing code for quality, correctness, and maintainability
tags: [code-review, quality, feedback]
---
You are reviewing existing code. Focus on correctness, readability, and
maintainability over style preferences. Call out bugs, security issues, and
logic errors first, then address design concerns, and finally suggest stylistic
improvements -- clearly distinguishing between these categories. When you
identify a problem, explain why it matters and suggest a concrete fix rather
than just pointing it out. Acknowledge what the code does well -- review is
feedback, not just criticism. Keep your comments proportional to the size
and risk of the change.
```

### Guidelines

- Write as direct instructions: "Focus on...", "Prioritize...", "When you..."
- Define the task clearly so the model adjusts its behavior accordingly.
- Include priorities: what matters most in this context.
- One paragraph is usually enough.

## Built-in contexts

| Name | Description |
|---|---|
| `architecture-review` | Evaluating system architecture and design decisions |
| `code-review` | Reviewing existing code for quality, correctness, and maintainability |
| `debugging` | Debugging and troubleshooting issues systematically |
| `documentation` | Writing or improving technical documentation |
| `estimation` | Estimating effort, complexity, and timelines |
| `greenfield-project` | Starting a new project from scratch |
| `incident-response` | Debugging production incidents under time pressure |
| `legacy-codebase` | Working with old, underdocumented, or messy codebases |
| `mentoring` | Teaching and guiding less experienced engineers |
| `migration` | Planning and executing system migrations |
| `onboarding` | Getting up to speed on a new codebase or team |
| `open-source` | Contributing to or maintaining open-source projects |
| `pair-programming` | Real-time collaborative coding and problem-solving |
| `performance-tuning` | Profiling and optimizing existing systems for performance |
| `post-mortem` | Incident analysis, root cause investigation, and action items |
| `prototyping` | Rapid experimentation and proof-of-concept development |
| `refactoring` | Improving existing code structure without changing behavior |
| `security-audit` | Reviewing systems for security vulnerabilities and threats |
