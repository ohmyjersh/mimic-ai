# Contributing

mimic ships with a curated set of built-in fragments. Contributions of new personas, skills, contexts, tones, and constraints are welcome.

## Adding a new fragment

1. Fork the repository: [github.com/ohmyjersh/mimic-ai](https://github.com/ohmyjersh/mimic-ai)
2. Create a markdown file in the appropriate `defaults/` subdirectory:

   ```
   defaults/
     personas/     # persona fragments
     skills/       # skill fragments
     contexts/     # context fragments
     tones/        # tone fragments
     constraints/  # constraint fragments
   ```

3. Use lowercase kebab-case for the filename: `my-new-fragment.md`
4. Include YAML frontmatter with `description` and `tags`:

   ```markdown
   ---
   description: One-line summary of what this fragment does
   tags: [relevant, tags, here]
   ---
   The prompt body goes here. Write in second person for personas and skills.
   Write as direct instructions for contexts, tones, and constraints.
   ```

5. Open a pull request.

## Fragment quality checklist

Before submitting, check that your fragment:

- [ ] Has a clear, accurate `description` in the frontmatter
- [ ] Has relevant `tags` for discoverability
- [ ] Is self-contained -- it should work with any combination of other fragments
- [ ] Uses second person ("You are...", "You write...") for personas and skills
- [ ] Uses direct instructions ("Be...", "Focus on...") for contexts, tones, and constraints
- [ ] Is one paragraph (two at most) -- concise fragments compose better than long ones
- [ ] Does not duplicate an existing built-in fragment

## Development

Clone and build:

```sh
git clone https://github.com/ohmyjersh/mimic-ai.git
cd mimic-ai
cargo build
```

Run tests:

```sh
cargo test
```

The test suite validates fragment parsing, registry layering, and prompt composition.

## Building the docs

The documentation uses [mdBook](https://rust-lang.github.io/mdBook/). To preview locally:

```sh
cd docs
mdbook serve
```
