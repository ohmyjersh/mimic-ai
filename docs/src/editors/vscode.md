# VS Code

VS Code supports MCP servers through its Copilot Chat extension. mimic also provides a dedicated extension with a visual [Prompt Studio](../prompt-studio.md) for composing prompts.

## MCP Setup (Copilot Chat)

Add mimic to your VS Code MCP configuration. Create or edit `.vscode/mcp.json` in your workspace:

```json
{
  "servers": {
    "mimic": {
      "command": "mimic",
      "type": "stdio"
    }
  }
}
```

Or add it to your user settings (`settings.json`):

```json
{
  "mcp": {
    "servers": {
      "mimic": {
        "command": "mimic",
        "type": "stdio"
      }
    }
  }
}
```

Once connected, use mimic tools in Copilot Chat. In the chat panel, you can reference mimic's compose tool:

> "Use the backend-engineer persona with go skills to review this code."

VS Code will call the compose tool through the MCP connection and apply the persona.

## Prompt Studio Extension

The mimic VS Code extension adds a visual interface for building prompts without touching the command line. See [Prompt Studio](../prompt-studio.md) for full documentation.

**Key features:**
- Drag-and-drop fragment selection across five categories
- Live prompt preview with word count
- Interactive relationship graph
- Persona-based recommendations
- Saveable templates for common workflows
- Insert composed prompts directly into the editor

Open it with `Cmd+Shift+P` / `Ctrl+Shift+P` and search for `Mimic: Open Prompt Studio`.
