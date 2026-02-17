# Cursor

[Cursor](https://cursor.com) is an AI-powered code editor with MCP support.

## Setup

Open Cursor settings and navigate to the MCP servers section. Add a new server with the following configuration:

```json
{
  "mcpServers": {
    "mimic": {
      "command": "mimic",
      "transport": "stdio"
    }
  }
}
```

Alternatively, create a `.cursor/mcp.json` file in your project root:

```json
{
  "mcpServers": {
    "mimic": {
      "command": "mimic",
      "transport": "stdio"
    }
  }
}
```

## Usage

Once connected, Cursor's AI features can call mimic's tools. In the chat panel, ask Cursor to compose a persona:

> "Use the frontend-engineer persona with react and typescript skills for this refactoring."

Cursor will call the compose tool and apply the persona to the conversation.
