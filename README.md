# agd

Dispatch tasks to other agent CLIs with tier-based fallback.

## Installation

### Windows (PowerShell)

User install:

```powershell
$env:APP_NAME="agd"; $env:REPO="superyngo/agd"; irm https://gist.githubusercontent.com/superyngo/a6b786af38b8b4c2ce15a70ae5387bd7/raw/gpinstaller.ps1 | iex
```

User uninstall:

```powershell
$env:APP_NAME="agd"; $env:REPO="superyngo/agd"; $env:UNINSTALL="true"; irm https://gist.githubusercontent.com/superyngo/a6b786af38b8b4c2ce15a70ae5387bd7/raw/gpinstaller.ps1 | iex
```

System install (requires Administrator):

```powershell
Start-Process pwsh -Verb RunAs -ArgumentList "-NoExit","-Command","`$env:APP_NAME='agd'; `$env:REPO='superyngo/agd'; irm https://gist.githubusercontent.com/superyngo/a6b786af38b8b4c2ce15a70ae5387bd7/raw/gpinstaller.ps1 | iex"
```

### Linux / macOS (Bash)

User install:

```bash
curl -fsSL https://gist.githubusercontent.com/superyngo/a6b786af38b8b4c2ce15a70ae5387bd7/raw/gpinstaller.sh \
  | APP_NAME="agd" REPO="superyngo/agd" bash
```

User uninstall:

```bash
curl -fsSL https://gist.githubusercontent.com/superyngo/a6b786af38b8b4c2ce15a70ae5387bd7/raw/gpinstaller.sh \
  | APP_NAME="agd" REPO="superyngo/agd" bash -s uninstall
```

System install (requires root):

```bash
sudo -E bash -c 'curl -fsSL https://gist.githubusercontent.com/superyngo/a6b786af38b8b4c2ce15a70ae5387bd7/raw/gpinstaller.sh \
  | APP_NAME="agd" REPO="superyngo/agd" bash'
```

## Usage

### Subcommands

| Command | Description |
|---|---|
| `agd detect` | List installed agent CLIs and their detected versions |
| `agd init` | Generate a config file from a JSON spec on stdin |
| `agd config <action>` | `show` / `path` / `edit` / `list` — inspect or open the active config |
| `agd dispatch -p "<prompt>" [--tier ID \| --agent ID] [--dry-run] [--verbose]` | Run agents in tier order (with round-robin) or target a single agent |
| `agd --version` | Print version |

### Configuration

`agd` reads the first config file found in this order:
1. `--config <path>` (explicit override)
2. `<git-root>/.config/agd.toml` (project-level)
3. `~/.config/agd.toml` (user-level)

Example `agd.toml`:

```toml
version = 1

[[tiers]]
id = "primary"

  [[tiers.agents]]
  id = "claude-claude"
  cli = "claude"
  model = "default"
  args = ["--dangerously-skip-permissions"]
  enabled = true
    [[tiers.agents.env]]
    type = "source"
    path = "~/.zshrc.d/cclaude.env"

  [[tiers.agents]]
  id = "antigravity-cli"
  cli = "agy"
  model = "default"
  args = ["--dangerously-skip-permissions"]
  enabled = true

  [[tiers.agents]]
  id = "claude-glm"
  cli = "claude"
  model = "default"
  args = ["--dangerously-skip-permissions"]
  enabled = true
    [[tiers.agents.env]]
    type = "source"
    path = "~/.zshrc.d/zclaude.env"
```

Each agent's `cli` field must match a top-level table key in `Resources/cli-templates.toml` (shipped with the binary; older installs using `config/cli-templates.toml` are still detected). `[[tiers.agents.env]]` entries support three types: `source` (shell-source a file before launching), `file` (read file contents into an env var), and `env` (copy from a parent env var).

### Common commands

```bash
# Where is my config?
agd config path

# Open the active config in $EDITOR
agd config edit

# Print the resolved config as TOML
agd config show

# Run primary tier — fall through agents in round-robin order
agd dispatch -p "summarize this PR" --tier primary

# Preview the command line for a single agent without spawning it
agd dispatch -p "summarize this PR" --agent claude-claude --dry-run

# Read prompt from a file
agd dispatch -f prompt.md --tier primary
```

### Environment variables

| Variable | Purpose |
|---|---|
| `AGD_TEMPLATES` | Override the path to `cli-templates.toml`. Useful for development or alternate installs. |
| `AGD_DEPTH` | Internal re-entry guard. Set automatically by `agd` when spawning child agents — **do not set manually**. |

## License

MIT
