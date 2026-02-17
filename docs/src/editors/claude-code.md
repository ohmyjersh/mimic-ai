# Claude Code

## Setup

Add mimic as an MCP server:

```sh
claude mcp add --transport stdio mimic -- mimic
```

This registers mimic with Claude Code using stdio transport. The server starts automatically when Claude Code launches.

## Verify

Check that mimic is connected:

```sh
claude mcp list
```

You should see `mimic` in the output.

## Usage

Once connected, Claude Code can call mimic's `compose` and `list` tools. You can ask Claude to use a specific persona directly in conversation:

> "Use the backend-engineer persona with rust and postgresql skills for this code review."

Claude will call the `compose` tool with the appropriate arguments and apply the resulting system prompt.

You can also ask Claude to list available fragments:

> "What personas and skills does mimic have available?"

## Configuration file

Alternatively, add mimic to your Claude Code MCP configuration file (`~/.claude/claude_desktop_config.json`):

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
