# dispatch-agent

Dispatch tasks to other agent CLIs with tier-based fallback.

## Installation

### Windows (PowerShell)

User install:

```powershell
$env:APP_NAME="dispatch-agent"; $env:REPO="superyngo/dispatch-agent"; irm https://gist.githubusercontent.com/superyngo/a6b786af38b8b4c2ce15a70ae5387bd7/raw/gpinstaller.ps1 | iex
```

User uninstall:

```powershell
$env:APP_NAME="dispatch-agent"; $env:REPO="superyngo/dispatch-agent"; $env:UNINSTALL="true"; irm https://gist.githubusercontent.com/superyngo/a6b786af38b8b4c2ce15a70ae5387bd7/raw/gpinstaller.ps1 | iex
```

System install (requires Administrator):

```powershell
Start-Process pwsh -Verb RunAs -ArgumentList "-NoExit","-Command","`$env:APP_NAME='dispatch-agent'; `$env:REPO='superyngo/dispatch-agent'; irm https://gist.githubusercontent.com/superyngo/a6b786af38b8b4c2ce15a70ae5387bd7/raw/gpinstaller.ps1 | iex"
```

### Linux / macOS (Bash)

User install:

```bash
curl -fsSL https://gist.githubusercontent.com/superyngo/a6b786af38b8b4c2ce15a70ae5387bd7/raw/gpinstaller.sh \
  | APP_NAME="dispatch-agent" REPO="superyngo/dispatch-agent" bash
```

User uninstall:

```bash
curl -fsSL https://gist.githubusercontent.com/superyngo/a6b786af38b8b4c2ce15a70ae5387bd7/raw/gpinstaller.sh \
  | APP_NAME="dispatch-agent" REPO="superyngo/dispatch-agent" bash -s uninstall
```

System install (requires root):

```bash
sudo -E bash -c 'curl -fsSL https://gist.githubusercontent.com/superyngo/a6b786af38b8b4c2ce15a70ae5387bd7/raw/gpinstaller.sh \
  | APP_NAME="dispatch-agent" REPO="superyngo/dispatch-agent" bash'
```

## Usage

...

## License

MIT
