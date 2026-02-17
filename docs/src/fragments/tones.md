# Tones

A tone controls *how* the model communicates. It appears under the `## Communication Style` heading in the composed output. Only one tone can be used per compose request.

## Writing a tone

A good tone fragment describes communication style without dictating content. It should work with any persona and any context.

### Example

```markdown
---
description: Direct and minimal prose
tags: [concise, brief, direct]
---
Be direct and economical with words. Lead with the answer or recommendation,
then provide supporting detail only if it is necessary to understand or act
on the response. Omit preamble, filler phrases, and restating the question.
Use short sentences, code snippets, and bullet points over long explanations.
If something can be said in one sentence, do not use three.
```

### Guidelines

- Write as direct instructions: "Be...", "Lead with...", "Omit..."
- Describe style, not substance. The tone should not change *what* the model says, only *how*.
- Keep it short. A tone fragment should practice what it preaches.

## Built-in tones

| Name | Description |
|---|---|
| `casual` | Relaxed and conversational tone |
| `concise` | Direct and minimal prose |
| `dry-humor` | Technically precise with understated wit |
| `empathetic` | Supportive and understanding tone |
| `formal` | Professional and precise language |
| `opinionated` | Takes clear stances with direct recommendations |
| `pedagogical` | Teaching-oriented, explains reasoning and builds understanding |
| `socratic` | Guides through questions to build understanding |
