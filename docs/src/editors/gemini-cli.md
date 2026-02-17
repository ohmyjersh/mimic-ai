# Gemini CLI

[Gemini CLI](https://github.com/google-gemini/gemini-cli) is Google's command-line AI agent.

## Setup

Add mimic to your Gemini CLI MCP configuration. Create or edit `~/.gemini/settings.json`:

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

Once configured, Gemini CLI can call mimic's `compose` and `list` tools. Ask Gemini to use a persona:

> "Use the devops-engineer persona with kubernetes and docker skills."

Gemini will call the compose tool and use the resulting prompt to guide its responses.
