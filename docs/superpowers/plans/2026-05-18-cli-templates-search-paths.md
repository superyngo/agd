# cli-templates.toml Platform Fallback Search Paths Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Extend `resolve_templates_path()` in `src/templates.rs` so that, after the existing search chain, `cli-templates.toml` is also looked up in a set of platform-specific install locations (wenget/Program Files/`/opt`/`/usr/local/bin`/etc.).

**Architecture:** Add a platform-gated helper `platform_fallback_candidates()` returning a `Vec<PathBuf>` of fully-resolved candidate paths (skipping any whose required env var is unset/empty). Add a small `find_first_existing()` helper for testability. Wire both into the existing resolver after the dev fallback. Update error message to list all checked paths.

**Tech Stack:** Rust, `std::env`, `anyhow`, existing `tempfile` for tests.

Spec: `docs/superpowers/specs/2026-05-18-cli-templates-search-paths-design.md`

---

## File Structure

- Modify: `src/templates.rs`
  - Add private helpers `candidate_from_env`, `find_first_existing`, `platform_fallback_candidates` (cfg-gated for unix/windows).
  - Extend `resolve_templates_path()` to use them and update error message.
  - Add tests covering the new behavior.
- Modify: `CHANGELOG.md` — append Unreleased entry.

No other files change. `src/detect.rs` and `src/config_cmd.rs` are out of scope (per spec).

---

## Task 1: Add `find_first_existing` helper with test

**Files:**
- Modify: `src/templates.rs` (add helper near `resolve_templates_path`, add test in `#[cfg(test)] mod tests`)

- [ ] **Step 1: Write the failing test**

Add inside `mod tests` in `src/templates.rs` (after `missing_file_error`):

```rust
    #[test]
    fn find_first_existing_returns_first_hit() {
        let dir = tempfile::tempdir().unwrap();
        let a = dir.path().join("a.toml");
        let b = dir.path().join("b.toml");
        std::fs::File::create(&b).unwrap();
        let result = super::find_first_existing(&[a.clone(), b.clone()]);
        assert_eq!(result, Some(b));
    }

    #[test]
    fn find_first_existing_returns_none_when_no_match() {
        let dir = tempfile::tempdir().unwrap();
        let a = dir.path().join("a.toml");
        let b = dir.path().join("b.toml");
        let result = super::find_first_existing(&[a, b]);
        assert_eq!(result, None);
    }
```

- [ ] **Step 2: Run tests to verify failure**

Run: `cargo test --lib templates::tests::find_first_existing -- --nocapture`
Expected: FAIL — `find_first_existing` not found in module `super`.

- [ ] **Step 3: Implement `find_first_existing`**

Add at top-level of `src/templates.rs` (just below `resolve_templates_path` declaration is fine, or before it):

```rust
#[allow(dead_code)]
fn find_first_existing(candidates: &[std::path::PathBuf]) -> Option<std::path::PathBuf> {
    candidates.iter().find(|p| p.exists()).cloned()
}
```

- [ ] **Step 4: Run tests to verify pass**

Run: `cargo test --lib templates::tests::find_first_existing`
Expected: 2 tests pass.

- [ ] **Step 5: Commit**

```bash
git add src/templates.rs
git commit -m "feat(templates): add find_first_existing helper"
```

---

## Task 2: Add `candidate_from_env` helper with test

**Files:**
- Modify: `src/templates.rs`

- [ ] **Step 1: Write the failing tests**

Add inside `mod tests`:

```rust
    #[test]
    fn candidate_from_env_returns_none_when_unset() {
        let _lock = ENV_MUTEX.lock().unwrap();
        // Use a name that is extremely unlikely to be set
        env::remove_var("DA_TEST_UNSET_VAR_XYZ");
        let result = super::candidate_from_env("DA_TEST_UNSET_VAR_XYZ", &["sub", "file.toml"]);
        assert!(result.is_none());
    }

    #[test]
    fn candidate_from_env_returns_none_when_empty() {
        let _lock = ENV_MUTEX.lock().unwrap();
        let _g = EnvGuard::set("DA_TEST_EMPTY_VAR_XYZ", "");
        let result = super::candidate_from_env("DA_TEST_EMPTY_VAR_XYZ", &["x"]);
        assert!(result.is_none());
    }

    #[test]
    fn candidate_from_env_joins_suffix() {
        let _lock = ENV_MUTEX.lock().unwrap();
        let dir = tempfile::tempdir().unwrap();
        let _g = EnvGuard::set("DA_TEST_BASE_XYZ", dir.path().to_str().unwrap());
        let result = super::candidate_from_env("DA_TEST_BASE_XYZ", &["a", "b.toml"]).unwrap();
        assert_eq!(result, dir.path().join("a").join("b.toml"));
    }
```

- [ ] **Step 2: Run tests to verify failure**

Run: `cargo test --lib templates::tests::candidate_from_env`
Expected: FAIL — `candidate_from_env` not found.

- [ ] **Step 3: Implement `candidate_from_env`**

Add at top-level of `src/templates.rs`:

```rust
#[allow(dead_code)]
fn candidate_from_env(var: &str, suffix: &[&str]) -> Option<std::path::PathBuf> {
    let base = env::var(var).ok()?;
    if base.is_empty() {
        return None;
    }
    let mut p = std::path::PathBuf::from(base);
    for s in suffix {
        p.push(s);
    }
    Some(p)
}
```

- [ ] **Step 4: Run tests to verify pass**

Run: `cargo test --lib templates::tests::candidate_from_env`
Expected: 3 tests pass.

- [ ] **Step 5: Commit**

```bash
git add src/templates.rs
git commit -m "feat(templates): add candidate_from_env helper"
```

---

## Task 3: Add `platform_fallback_candidates` (Unix) with test

**Files:**
- Modify: `src/templates.rs`

- [ ] **Step 1: Write the failing test (Unix-gated)**

Add inside `mod tests`:

```rust
    #[cfg(unix)]
    #[test]
    fn platform_fallback_candidates_unix_uses_home_and_absolutes() {
        let _lock = ENV_MUTEX.lock().unwrap();
        let dir = tempfile::tempdir().unwrap();
        let _g = EnvGuard::set("HOME", dir.path().to_str().unwrap());
        let candidates = super::platform_fallback_candidates();

        let home = dir.path();
        let expected_home_wenget = home
            .join(".wenget/apps/dispatch-agent/config/cli-templates.toml");
        let expected_home_local = home.join(".local/bin/config/cli-templates.toml");
        let expected_opt = std::path::PathBuf::from(
            "/opt/wenget/apps/dispatch-agent/config/cli-templates.toml",
        );
        let expected_usr = std::path::PathBuf::from(
            "/usr/local/bin/config/cli-templates.toml",
        );

        assert!(
            candidates.contains(&expected_home_wenget),
            "missing {} in {:?}",
            expected_home_wenget.display(),
            candidates
        );
        assert!(candidates.contains(&expected_home_local));
        assert!(candidates.contains(&expected_opt));
        assert!(candidates.contains(&expected_usr));

        // Order: HOME entries come before absolute /opt and /usr/local entries
        let pos = |needle: &std::path::PathBuf| candidates.iter().position(|c| c == needle).unwrap();
        assert!(pos(&expected_home_wenget) < pos(&expected_opt));
        assert!(pos(&expected_home_local) < pos(&expected_opt));
        assert!(pos(&expected_opt) < pos(&expected_usr));
    }

    #[cfg(unix)]
    #[test]
    fn platform_fallback_candidates_unix_skips_when_home_unset() {
        let _lock = ENV_MUTEX.lock().unwrap();
        let _g = EnvGuard::set("HOME", "");
        let candidates = super::platform_fallback_candidates();
        // Absolute paths still present
        assert!(candidates.iter().any(|c| c.starts_with("/opt/wenget")));
        assert!(candidates.iter().any(|c| c.starts_with("/usr/local/bin")));
        // No path should contain ".wenget/apps/dispatch-agent" rooted in empty/HOME
        assert!(
            !candidates.iter().any(|c| c.to_string_lossy().starts_with(".wenget")),
            "candidates leaked relative HOME path: {:?}",
            candidates
        );
    }
```

- [ ] **Step 2: Run tests to verify failure**

Run: `cargo test --lib templates::tests::platform_fallback_candidates_unix`
Expected: FAIL — `platform_fallback_candidates` not found.

- [ ] **Step 3: Implement `platform_fallback_candidates` for Unix**

Add at top-level of `src/templates.rs`:

```rust
#[allow(dead_code)]
#[cfg(unix)]
fn platform_fallback_candidates() -> Vec<std::path::PathBuf> {
    let mut out = Vec::new();
    if let Some(p) = candidate_from_env(
        "HOME",
        &[".wenget", "apps", "dispatch-agent", "config", "cli-templates.toml"],
    ) {
        out.push(p);
    }
    if let Some(p) = candidate_from_env(
        "HOME",
        &[".local", "bin", "config", "cli-templates.toml"],
    ) {
        out.push(p);
    }
    out.push(std::path::PathBuf::from(
        "/opt/wenget/apps/dispatch-agent/config/cli-templates.toml",
    ));
    out.push(std::path::PathBuf::from(
        "/usr/local/bin/config/cli-templates.toml",
    ));
    out
}
```

- [ ] **Step 4: Run tests to verify pass**

Run: `cargo test --lib templates::tests::platform_fallback_candidates_unix`
Expected: 2 tests pass.

- [ ] **Step 5: Commit**

```bash
git add src/templates.rs
git commit -m "feat(templates): add unix platform fallback candidates"
```

---

## Task 4: Add `platform_fallback_candidates` (Windows) with test

**Files:**
- Modify: `src/templates.rs`

- [ ] **Step 1: Write the failing test (Windows-gated)**

Add inside `mod tests`:

```rust
    #[cfg(windows)]
    #[test]
    fn platform_fallback_candidates_windows_uses_env_vars() {
        let _lock = ENV_MUTEX.lock().unwrap();
        let dir = tempfile::tempdir().unwrap();
        let _u = EnvGuard::set("USERPROFILE", dir.path().to_str().unwrap());
        let _l = EnvGuard::set("LOCALAPPDATA", dir.path().to_str().unwrap());
        let _w = EnvGuard::set("ProgramW6432", dir.path().to_str().unwrap());
        let _p = EnvGuard::set("ProgramFiles", dir.path().to_str().unwrap());

        let candidates = super::platform_fallback_candidates();
        let base = dir.path();

        let exp_userprofile = base
            .join(".wenget").join("apps").join("dispatch-agent").join("config").join("cli-templates.toml");
        let exp_localappdata = base
            .join("Programs").join("dispatch-agent").join("config").join("cli-templates.toml");
        let exp_progw6432 = base
            .join("wenget").join("app").join("dispatch-agent").join("config").join("cli-templates.toml");
        let exp_progfiles = base
            .join("gpinstall").join("config").join("cli-templates.toml");

        assert!(candidates.contains(&exp_userprofile));
        assert!(candidates.contains(&exp_localappdata));
        assert!(candidates.contains(&exp_progw6432));
        assert!(candidates.contains(&exp_progfiles));

        let pos = |needle: &std::path::PathBuf| candidates.iter().position(|c| c == needle).unwrap();
        assert!(pos(&exp_userprofile) < pos(&exp_localappdata));
        assert!(pos(&exp_localappdata) < pos(&exp_progw6432));
        assert!(pos(&exp_progw6432) < pos(&exp_progfiles));
    }

    #[cfg(windows)]
    #[test]
    fn platform_fallback_candidates_windows_skips_unset_vars() {
        let _lock = ENV_MUTEX.lock().unwrap();
        let _u = EnvGuard::set("USERPROFILE", "");
        let _l = EnvGuard::set("LOCALAPPDATA", "");
        let _w = EnvGuard::set("ProgramW6432", "");
        let _p = EnvGuard::set("ProgramFiles", "");
        let candidates = super::platform_fallback_candidates();
        assert!(candidates.is_empty(), "expected empty, got {:?}", candidates);
    }
```

- [ ] **Step 2: Run tests to verify failure (on Windows host) / verify compile (on Unix host)**

Run on Windows: `cargo test --lib templates::tests::platform_fallback_candidates_windows`
Expected on Windows: FAIL — symbol not found.

Run on macOS/Linux: `cargo build --lib --tests` — must still compile (cfg-gated test isn't built).
Expected: build succeeds.

- [ ] **Step 3: Implement `platform_fallback_candidates` for Windows**

Add at top-level of `src/templates.rs`:

```rust
#[allow(dead_code)]
#[cfg(windows)]
fn platform_fallback_candidates() -> Vec<std::path::PathBuf> {
    let mut out = Vec::new();
    if let Some(p) = candidate_from_env(
        "USERPROFILE",
        &[".wenget", "apps", "dispatch-agent", "config", "cli-templates.toml"],
    ) {
        out.push(p);
    }
    if let Some(p) = candidate_from_env(
        "LOCALAPPDATA",
        &["Programs", "dispatch-agent", "config", "cli-templates.toml"],
    ) {
        out.push(p);
    }
    if let Some(p) = candidate_from_env(
        "ProgramW6432",
        &["wenget", "app", "dispatch-agent", "config", "cli-templates.toml"],
    ) {
        out.push(p);
    }
    if let Some(p) = candidate_from_env(
        "ProgramFiles",
        &["gpinstall", "config", "cli-templates.toml"],
    ) {
        out.push(p);
    }
    out
}
```

- [ ] **Step 4: Verify build on current host**

Run: `cargo build --lib --tests`
Expected: succeeds. (Windows-only tests run when host is Windows.)

- [ ] **Step 5: Commit**

```bash
git add src/templates.rs
git commit -m "feat(templates): add windows platform fallback candidates"
```

---

## Task 5: Wire platform fallback into `resolve_templates_path` and update error message

**Files:**
- Modify: `src/templates.rs:24-45` (`resolve_templates_path` body and error)

- [ ] **Step 1: Write the failing test**

Add inside `mod tests`:

```rust
    #[cfg(unix)]
    #[test]
    fn resolve_templates_path_uses_unix_fallback() {
        let _lock = ENV_MUTEX.lock().unwrap();
        let dir = tempfile::tempdir().unwrap();
        // Build $HOME/.wenget/apps/dispatch-agent/config/cli-templates.toml
        let nested = dir
            .path()
            .join(".wenget/apps/dispatch-agent/config");
        std::fs::create_dir_all(&nested).unwrap();
        let target = nested.join("cli-templates.toml");
        std::fs::write(&target, "[cli]\nprompt_flag = \"-p\"\n").unwrap();

        let _h = EnvGuard::set("HOME", dir.path().to_str().unwrap());
        // Ensure earlier links in the chain miss:
        env::remove_var("DISPATCH_AGENT_TEMPLATES");
        // exe_dir/config/cli-templates.toml normally won't exist for cargo-test binaries;
        // CARGO_MANIFEST_DIR/config/cli-templates.toml DOES exist in this repo, so
        // resolve_templates_path() will short-circuit there. We therefore validate the
        // fallback by checking find_first_existing(platform_fallback_candidates()) directly.
        let got = super::find_first_existing(&super::platform_fallback_candidates())
            .expect("expected fallback hit");
        assert_eq!(got, target);
    }

    #[test]
    fn resolve_templates_path_error_lists_paths() {
        let _lock = ENV_MUTEX.lock().unwrap();
        env::remove_var("DISPATCH_AGENT_TEMPLATES");
        // We cannot force exe_dir and CARGO_MANIFEST_DIR misses without filesystem
        // gymnastics; instead, exercise the error formatter directly.
        let msg = super::format_not_found_error(&[
            std::path::PathBuf::from("/tmp/a/cli-templates.toml"),
            std::path::PathBuf::from("/tmp/b/cli-templates.toml"),
        ]);
        assert!(msg.contains("/tmp/a/cli-templates.toml"));
        assert!(msg.contains("/tmp/b/cli-templates.toml"));
        assert!(msg.contains("DISPATCH_AGENT_TEMPLATES"));
    }
```

- [ ] **Step 2: Run tests to verify failure**

Run: `cargo test --lib templates::tests::resolve_templates_path`
Expected: FAIL — `format_not_found_error` not found (and on Unix, the fallback test relies on `platform_fallback_candidates` already implemented in Task 3, so should pass already; if it does, that's fine).

- [ ] **Step 3: Refactor `resolve_templates_path` to use helpers and add `format_not_found_error`**

Replace the existing `resolve_templates_path` (lines 24–45) with:

```rust
#[allow(dead_code)]
fn resolve_templates_path() -> anyhow::Result<std::path::PathBuf> {
    let mut checked: Vec<std::path::PathBuf> = Vec::new();

    if let Ok(p) = env::var("DISPATCH_AGENT_TEMPLATES") {
        let path = std::path::PathBuf::from(&p);
        if path.exists() {
            return Ok(path);
        }
        checked.push(path);
    }

    if let Ok(exe) = env::current_exe() {
        if let Some(exe_dir) = exe.parent() {
            let p = exe_dir.join("config/cli-templates.toml");
            if p.exists() {
                return Ok(p);
            }
            checked.push(p);
        }
    }

    let dev_path =
        std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("config/cli-templates.toml");
    if dev_path.exists() {
        return Ok(dev_path);
    }
    checked.push(dev_path);

    let platform = platform_fallback_candidates();
    if let Some(hit) = find_first_existing(&platform) {
        return Ok(hit);
    }
    checked.extend(platform);

    Err(anyhow!("{}", format_not_found_error(&checked)))
}

#[allow(dead_code)]
fn format_not_found_error(checked: &[std::path::PathBuf]) -> String {
    let mut s = String::from(
        "cli-templates.toml not found. Set DISPATCH_AGENT_TEMPLATES to override. Searched:\n",
    );
    for p in checked {
        s.push_str("  ");
        s.push_str(&p.display().to_string());
        s.push('\n');
    }
    s
}
```

- [ ] **Step 4: Run all template tests to verify pass**

Run: `cargo test --lib templates::tests`
Expected: all pass (including the existing `missing_file_error` — it asserts on `"reading templates file"` from `load_templates`, which still applies when `DISPATCH_AGENT_TEMPLATES` points to a missing file, because that branch now still returns the path and then `load_templates` fails reading it — wait, see Step 4a).

- [ ] **Step 4a: Verify existing `missing_file_error` still passes**

The original code returned the env-supplied path even when it didn't exist (so `load_templates` then failed at `read_to_string`). The new code only returns the env path if it `exists()`. If the env var points to a non-existent file, we now fall through. This would BREAK `missing_file_error`.

Update the env-var branch to preserve original behavior — return the env-supplied path **unconditionally** so that downstream `read_to_string` produces the existing error:

Replace the env-var block in `resolve_templates_path` with:

```rust
    if let Ok(p) = env::var("DISPATCH_AGENT_TEMPLATES") {
        return Ok(std::path::PathBuf::from(p));
    }
```

(This matches the original behavior at `src/templates.rs:25-27`.)

Re-run: `cargo test --lib templates::tests`
Expected: all pass.

- [ ] **Step 5: Run full test suite**

Run: `cargo test`
Expected: all pass.

- [ ] **Step 6: Commit**

```bash
git add src/templates.rs
git commit -m "feat(templates): search platform fallback paths for cli-templates.toml"
```

---

## Task 6: Update CHANGELOG

**Files:**
- Modify: `CHANGELOG.md` — append entry under `## [Unreleased]`

- [ ] **Step 1: Edit CHANGELOG**

Replace the `## [Unreleased]` block (currently empty) with:

```markdown
## [Unreleased]

### Added
- `cli-templates.toml` resolver now searches platform-specific fallback paths after the existing chain (env var → exe dir → dev manifest). On Unix: `$HOME/.wenget/apps/dispatch-agent/config/`, `$HOME/.local/bin/config/`, `/opt/wenget/apps/dispatch-agent/config/`, `/usr/local/bin/config/`. On Windows: `%USERPROFILE%\.wenget\apps\dispatch-agent\config\`, `%LOCALAPPDATA%\Programs\dispatch-agent\config\`, `%ProgramW6432%\wenget\app\dispatch-agent\config\`, `%ProgramFiles%\gpinstall\config\`. Candidates whose required env var is unset/empty are skipped. The "not found" error now lists every path checked.
```

- [ ] **Step 2: Commit**

```bash
git add CHANGELOG.md
git commit -m "docs(changelog): note cli-templates.toml fallback search paths"
```

---

## Task 7: Final verification

- [ ] **Step 1: Build and test**

Run:
```bash
cargo build --release
cargo test
cargo clippy --all-targets -- -D warnings
```
Expected: all succeed; no clippy errors.

- [ ] **Step 2: Manual smoke (Unix host only)**

```bash
# Verify the search behavior end-to-end on the current platform
mkdir -p /tmp/fake_home/.wenget/apps/dispatch-agent/config
cp config/cli-templates.toml /tmp/fake_home/.wenget/apps/dispatch-agent/config/cli-templates.toml
# Move dev fallback out of the way temporarily by running outside the project workdir
# (Or simply trust the unit test resolve_templates_path_uses_unix_fallback.)
```

This step is optional — the unit test in Task 5 covers it. Skip if running purely from `cargo test`.

- [ ] **Step 3: Confirm clean tree**

Run: `git status`
Expected: clean (all changes committed).

---

## Self-Review Results

**Spec coverage**

- §"變更後搜尋順序" (Unix/Windows path lists) → Tasks 3, 4.
- §"行為規則" (skip on unset, return first hit, error lists checked) → Tasks 1, 2, 5.
- §"實作" `candidate_from_env`, `candidate_absolute`, `platform_fallback_candidates` → Tasks 2, 3, 4 (`candidate_absolute` from spec was inlined as plain `PathBuf::from(...)` since it provided no abstraction; documented here as a deliberate simplification).
- §"實作" updated error message → Task 5.
- §"不變動" `DISPATCH_AGENT_TEMPLATES` behavior preserved → Task 5 Step 4a explicitly preserves the original "return path even if missing" behavior.
- §"測試" three test categories → Tasks 1, 2, 3, 4, 5 cover them.
- §"文件" CHANGELOG → Task 6. README explicitly out of scope.

**Placeholder scan:** None. Every step contains concrete code or commands.

**Type consistency:** Function names used consistently across tasks: `find_first_existing`, `candidate_from_env`, `platform_fallback_candidates`, `format_not_found_error`, `resolve_templates_path`.

**Deviation note:** Spec mentioned `candidate_absolute` helper. Plan inlines `PathBuf::from(...)` for the two absolute paths since wrapping a single constructor call would be ceremony. If the engineer prefers symmetry with `candidate_from_env`, adding it is harmless but not required.
