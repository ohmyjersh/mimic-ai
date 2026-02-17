# mimic — VS Code Extension

Compose LLM persona prompts from reusable fragments, right inside VS Code.

## Features

- **Compose a Persona Prompt** (`mimic.compose`): Walk through a guided multi-step picker to select a persona, skills, context, tone, and constraints. The composed prompt is inserted into your active editor or opened in a new document.
- **List Available Fragments** (`mimic.list`): Browse all available fragments grouped by category (persona, skill, context, tone, constraint).

## Requirements

- The `mimic` binary must be installed and available on your `PATH`, or configured via the `mimic.binaryPath` setting.
- The extension communicates with mimic over stdio using the MCP protocol (JSON-RPC).

## Configuration

| Setting            | Default   | Description                  |
|--------------------|-----------|------------------------------|
| `mimic.binaryPath` | `"mimic"` | Path to the mimic binary.    |

## Usage

1. Open the Command Palette (`Cmd+Shift+P` / `Ctrl+Shift+P`).
2. Run **Mimic: Compose a Persona Prompt** to build a prompt step by step.
3. Run **Mimic: List Available Fragments** to browse all loaded fragments.

## Development

```sh
cd extensions/vscode
npm install
npm run compile
```

To test, press `F5` in VS Code to launch an Extension Development Host.
