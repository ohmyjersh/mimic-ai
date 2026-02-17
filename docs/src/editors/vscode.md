# VS Code

VS Code supports MCP servers through its Copilot Chat extension.

## Setup

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

## Usage

Once connected, use mimic tools in Copilot Chat. In the chat panel, you can reference mimic's compose tool:

> "Use the backend-engineer persona with go skills to review this code."

VS Code will call the compose tool through the MCP connection and apply the persona.
