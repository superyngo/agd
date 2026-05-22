# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## v2.0.1 — 2026-05-22

### Changed
- Bundled templates relocated from `config/cli-templates.toml` to `Resources/cli-templates.toml`. Release archives (`.tar.gz` / `.zip`) and the in-repo source tree both use the new path. Startup resolver searches `Resources/` first then falls back to `config/` at every install location for backward compatibility with older installs.

## v2.0.0 — 2026-05-20

**Breaking:** project renamed from `dispatch-agent` to `agd` (agent dispatcher).

### Renamed
- Binary: `dispatch-agent` → `agd`
- Cargo crate: `dispatch-agent` → `agd`
- GitHub repo: `superyngo/dispatch-agent` → `superyngo/agd`
- Environment variables: `DISPATCH_AGENT_TEMPLATES` → `AGD_TEMPLATES`, `DISPATCH_AGENT_DEPTH` → `AGD_DEPTH`
- Config files: `~/.config/dispatch-agent.toml` → `~/.config/agd.toml`, `<git-root>/.config/dispatch-agent.toml` → `<git-root>/.config/agd.toml`
- Cache directory: `<cache>/dispatch-agent/` → `<cache>/agd/`
- Fallback install paths in `templates::platform_fallback_candidates`

No migration shim is provided. Existing local installs must be reinstalled and reconfigured.

### Added
- `agd --version` flag (prints `agd 2.0.0`)
- Template-resolution warnings and errors now include the absolute path to the loaded `cli-templates.toml`
- New `[agy]` template for the antigravity CLI in shipped `cli-templates.toml`

### Changed
- `[gemini-npx]` template now enables version probing (`version_flag = "--version"`) and no longer passes `--skip-trust`
- `load_templates()` now returns `(IndexMap, PathBuf)`
- README `## Usage` is now complete: subcommand reference, configuration example, environment variables

### Fixed
- Misleading inline comment `# detect reports version as null` removed from all seven `version_flag = "--version"` declarations in `cli-templates.toml`

## [v1.1.2] - 2026-05-18

### Added
- `cli-templates.toml` resolver now searches platform-specific fallback paths after the existing chain (env var → exe dir → dev manifest). On Unix: `$HOME/.wenget/apps/dispatch-agent/config/`, `$HOME/.local/bin/config/`, `/opt/wenget/apps/dispatch-agent/config/`, `/usr/local/bin/config/`. On Windows: `%USERPROFILE%\.wenget\apps\dispatch-agent\config\`, `%LOCALAPPDATA%\Programs\dispatch-agent\config\`, `%ProgramW6432%\wenget\app\dispatch-agent\config\`, `%ProgramFiles%\gpinstall\config\`. Candidates whose required env var is unset/empty are skipped. The "not found" error now lists every path checked.
- `candidate_from_env` helper to build path candidates from environment variables.
- `find_first_existing` helper to resolve the first existing path from a candidate list.

### Fixed
- Stub platform fallback for non-Unix/non-Windows targets; assert first-hit ordering in tests.

### CI
- Release archives now bundle `config/cli-templates.toml` alongside the binary.
- Added preflight check ensuring `config/cli-templates.toml` exists before packaging.
- Unified upload-artifact steps via `matrix.archive_ext` (`.tar.gz` / `.zip`).
- Added archive content verification step to build job.
- Added installation instructions to release body template.

## [v0.1.1] - 2026-05-15

### Fixed
- Gate `dispatch::process::unix` / `windows` submodules by target OS so Windows builds no longer fail trying to compile unix-only code (libc `setsid`/`killpg`, `pre_exec`, `signal_hook::iterator`).
- Add Windows stub for `start_signal_watcher`.

## [v0.1.0] - 2026-05-15

### Added
- Initial release of `dispatch-agent`: dispatch tasks to other agent CLIs with tier-based fallback.
- `init`, `detect`, `config`, and `dispatch` subcommands.
- CLI templates and round-robin tier state.
- GitHub Actions release workflow for multi-platform binary builds (Linux/Windows/macOS).
