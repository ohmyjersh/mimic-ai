# Codex CLI

[Codex CLI](https://github.com/openai/codex) is OpenAI's open-source coding agent for the terminal.

## Setup

Add mimic to your Codex CLI MCP configuration. Create or edit `~/.codex/config.json`:

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

Once configured, Codex CLI can call mimic's `compose` and `list` tools during conversations. Ask Codex to adopt a persona:

> "Use the fullstack-engineer persona with typescript and react skills."

Codex will call the compose tool and apply the resulting prompt.
