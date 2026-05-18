# Release Bundle Config Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Modify `.github/workflows/release.yml` so every release artifact bundles `config/cli-templates.toml` alongside the binary — `.tar.gz` for Linux/macOS, `.zip` for Windows.

**Architecture:** Each build job stages the binary and `config/cli-templates.toml` into a temporary `staging/` directory, packages it into the platform-appropriate archive, self-verifies the archive contents, then uploads. The release job collects all archives uniformly. A `.gitattributes` entry enforces LF line endings for the config file on all platforms.

**Tech Stack:** GitHub Actions YAML, bash (Linux/macOS steps), PowerShell Core / pwsh (Windows steps), `tar`, `Compress-Archive`

---

## Files Modified / Created

| File | Action | What changes |
|------|--------|-------------|
| `.gitattributes` | **Create** | Enforce `eol=lf` for `config/cli-templates.toml` |
| `.github/workflows/release.yml` | **Modify** | Matrix entries, packaging steps, verification steps, upload step, release job steps, release body |

---

### Task 1: Create `.gitattributes`

**Files:**
- Create: `.gitattributes`

- [ ] **Step 1: Create the file**

```
config/cli-templates.toml text eol=lf
```

Full command:
```bash
cat > .gitattributes << 'EOF'
config/cli-templates.toml text eol=lf
EOF
```

- [ ] **Step 2: Verify**

```bash
cat .gitattributes
```
Expected output:
```
config/cli-templates.toml text eol=lf
```

- [ ] **Step 3: Commit**

```bash
git add .gitattributes
git commit -m "chore: add .gitattributes to enforce LF for config/cli-templates.toml

Prevents Windows actions/checkout from converting LF→CRLF, which would
cause different SHA256 checksums for the same file across platforms.

Co-authored-by: Copilot <223556219+Copilot@users.noreply.github.com>"
```

---

### Task 2: Add `archive_ext` to all 10 Linux/macOS matrix entries

**Files:**
- Modify: `.github/workflows/release.yml:31-104`

This task adds `archive_ext: tar.gz` to each of the 8 Linux and 2 macOS targets. No other fields change. Do all 10 edits, then validate and commit once.

- [ ] **Step 1: Add `archive_ext: tar.gz` to the 8 Linux entries**

Make the following replacements in `.github/workflows/release.yml`:

```yaml
# Entry 1 — x86_64-unknown-linux-gnu (line ~34)
# BEFORE:
          - os: ubuntu-latest
            target: x86_64-unknown-linux-gnu
            artifact_name: dispatch-agent
            asset_name: dispatch-agent-linux-x86_64
# AFTER:
          - os: ubuntu-latest
            target: x86_64-unknown-linux-gnu
            artifact_name: dispatch-agent
            asset_name: dispatch-agent-linux-x86_64
            archive_ext: tar.gz
```

```yaml
# Entry 2 — i686-unknown-linux-gnu
# BEFORE:
          - os: ubuntu-latest
            target: i686-unknown-linux-gnu
            artifact_name: dispatch-agent
            asset_name: dispatch-agent-linux-i686
# AFTER:
          - os: ubuntu-latest
            target: i686-unknown-linux-gnu
            artifact_name: dispatch-agent
            asset_name: dispatch-agent-linux-i686
            archive_ext: tar.gz
```

```yaml
# Entry 3 — x86_64-unknown-linux-musl
# BEFORE:
          - os: ubuntu-latest
            target: x86_64-unknown-linux-musl
            artifact_name: dispatch-agent
            asset_name: dispatch-agent-linux-x86_64-musl
# AFTER:
          - os: ubuntu-latest
            target: x86_64-unknown-linux-musl
            artifact_name: dispatch-agent
            asset_name: dispatch-agent-linux-x86_64-musl
            archive_ext: tar.gz
```

```yaml
# Entry 4 — armv7-unknown-linux-gnueabihf
# BEFORE:
          - os: ubuntu-latest
            target: armv7-unknown-linux-gnueabihf
            artifact_name: dispatch-agent
            asset_name: dispatch-agent-linux-armv7
# AFTER:
          - os: ubuntu-latest
            target: armv7-unknown-linux-gnueabihf
            artifact_name: dispatch-agent
            asset_name: dispatch-agent-linux-armv7
            archive_ext: tar.gz
```

```yaml
# Entry 5 — aarch64-unknown-linux-gnu
# BEFORE:
          - os: ubuntu-latest
            target: aarch64-unknown-linux-gnu
            artifact_name: dispatch-agent
            asset_name: dispatch-agent-linux-aarch64
# AFTER:
          - os: ubuntu-latest
            target: aarch64-unknown-linux-gnu
            artifact_name: dispatch-agent
            asset_name: dispatch-agent-linux-aarch64
            archive_ext: tar.gz
```

```yaml
# Entry 6 — aarch64-unknown-linux-musl (has extra cflags/cc fields — add archive_ext after asset_name)
# BEFORE:
          - os: ubuntu-latest
            target: aarch64-unknown-linux-musl
            artifact_name: dispatch-agent
            asset_name: dispatch-agent-linux-aarch64-musl
            cflags: "-U_FORTIFY_SOURCE"
            cc: "aarch64-linux-gnu-gcc"
# AFTER:
          - os: ubuntu-latest
            target: aarch64-unknown-linux-musl
            artifact_name: dispatch-agent
            asset_name: dispatch-agent-linux-aarch64-musl
            archive_ext: tar.gz
            cflags: "-U_FORTIFY_SOURCE"
            cc: "aarch64-linux-gnu-gcc"
```

```yaml
# Entry 7 — i686-unknown-linux-musl
# BEFORE:
          - os: ubuntu-latest
            target: i686-unknown-linux-musl
            artifact_name: dispatch-agent
            asset_name: dispatch-agent-linux-i686-musl
# AFTER:
          - os: ubuntu-latest
            target: i686-unknown-linux-musl
            artifact_name: dispatch-agent
            asset_name: dispatch-agent-linux-i686-musl
            archive_ext: tar.gz
```

```yaml
# Entry 8 — armv7-unknown-linux-musleabihf
# BEFORE:
          - os: ubuntu-latest
            target: armv7-unknown-linux-musleabihf
            artifact_name: dispatch-agent
            asset_name: dispatch-agent-linux-armv7-musl
# AFTER:
          - os: ubuntu-latest
            target: armv7-unknown-linux-musleabihf
            artifact_name: dispatch-agent
            asset_name: dispatch-agent-linux-armv7-musl
            archive_ext: tar.gz
```

- [ ] **Step 2: Add `archive_ext: tar.gz` to the 2 macOS entries**

```yaml
# Entry 9 — x86_64-apple-darwin
# BEFORE:
          - os: macos-15-intel
            target: x86_64-apple-darwin
            artifact_name: dispatch-agent
            asset_name: dispatch-agent-macos-x86_64
# AFTER:
          - os: macos-15-intel
            target: x86_64-apple-darwin
            artifact_name: dispatch-agent
            asset_name: dispatch-agent-macos-x86_64
            archive_ext: tar.gz
```

```yaml
# Entry 10 — aarch64-apple-darwin
# BEFORE:
          - os: macos-latest
            target: aarch64-apple-darwin
            artifact_name: dispatch-agent
            asset_name: dispatch-agent-macos-aarch64
# AFTER:
          - os: macos-latest
            target: aarch64-apple-darwin
            artifact_name: dispatch-agent
            asset_name: dispatch-agent-macos-aarch64
            archive_ext: tar.gz
```

- [ ] **Step 3: Validate YAML syntax**

```bash
python3 -c "import yaml; yaml.safe_load(open('.github/workflows/release.yml')); print('YAML valid')"
```
Expected: `YAML valid`

- [ ] **Step 4: Commit**

```bash
git add .github/workflows/release.yml
git commit -m "ci: add archive_ext: tar.gz to all Linux and macOS matrix entries

Co-authored-by: Copilot <223556219+Copilot@users.noreply.github.com>"
```

---

### Task 3: Update the 3 Windows matrix entries

**Files:**
- Modify: `.github/workflows/release.yml:66-95`

Removes `.exe` suffix from `asset_name` and adds `archive_ext: zip`.

- [ ] **Step 1: Update x86_64 Windows entry**

```yaml
# BEFORE:
          - os: windows-latest
            target: x86_64-pc-windows-msvc
            artifact_name: dispatch-agent.exe
            asset_name: dispatch-agent-windows-x86_64.exe
            rustflags: "-C target-feature=+crt-static"
            opt_level: "3"
            lto: "thin"
            strip: "false"
            codegen_units: "16"
            panic: "unwind"
# AFTER:
          - os: windows-latest
            target: x86_64-pc-windows-msvc
            artifact_name: dispatch-agent.exe
            asset_name: dispatch-agent-windows-x86_64
            archive_ext: zip
            rustflags: "-C target-feature=+crt-static"
            opt_level: "3"
            lto: "thin"
            strip: "false"
            codegen_units: "16"
            panic: "unwind"
```

- [ ] **Step 2: Update i686 Windows entry**

```yaml
# BEFORE:
          - os: windows-latest
            target: i686-pc-windows-msvc
            artifact_name: dispatch-agent.exe
            asset_name: dispatch-agent-windows-i686.exe
            rustflags: "-C target-feature=+crt-static"
            opt_level: "3"
            lto: "thin"
            strip: "false"
            codegen_units: "16"
            panic: "unwind"
# AFTER:
          - os: windows-latest
            target: i686-pc-windows-msvc
            artifact_name: dispatch-agent.exe
            asset_name: dispatch-agent-windows-i686
            archive_ext: zip
            rustflags: "-C target-feature=+crt-static"
            opt_level: "3"
            lto: "thin"
            strip: "false"
            codegen_units: "16"
            panic: "unwind"
```

- [ ] **Step 3: Update aarch64 Windows entry**

```yaml
# BEFORE:
          - os: windows-latest
            target: aarch64-pc-windows-msvc
            artifact_name: dispatch-agent.exe
            asset_name: dispatch-agent-windows-aarch64.exe
            rustflags: "-C target-feature=+crt-static"
            opt_level: "3"
            lto: "thin"
            strip: "false"
            codegen_units: "16"
            panic: "unwind"
# AFTER:
          - os: windows-latest
            target: aarch64-pc-windows-msvc
            artifact_name: dispatch-agent.exe
            asset_name: dispatch-agent-windows-aarch64
            archive_ext: zip
            rustflags: "-C target-feature=+crt-static"
            opt_level: "3"
            lto: "thin"
            strip: "false"
            codegen_units: "16"
            panic: "unwind"
```

- [ ] **Step 4: Validate YAML syntax**

```bash
python3 -c "import yaml; yaml.safe_load(open('.github/workflows/release.yml')); print('YAML valid')"
```
Expected: `YAML valid`

- [ ] **Step 5: Commit**

```bash
git add .github/workflows/release.yml
git commit -m "ci: update Windows matrix entries — remove .exe from asset_name, add archive_ext: zip

This is a breaking change: Windows release artifacts will now be .zip
files instead of bare .exe files.

Co-authored-by: Copilot <223556219+Copilot@users.noreply.github.com>"
```

---

### Task 4: Add preflight config file check step

**Files:**
- Modify: `.github/workflows/release.yml` (steps section, after "Checkout code")

The step uses `shell: bash` so it runs identically on all 3 OS types (GitHub Actions Windows runners have bash available).

- [ ] **Step 1: Add the step after "Checkout code"**

```yaml
# BEFORE (lines ~107-109):
      - name: Checkout code
        uses: actions/checkout@v4

      - name: Setup Rust
# AFTER:
      - name: Checkout code
        uses: actions/checkout@v4

      - name: Verify config file exists
        shell: bash
        run: test -f config/cli-templates.toml

      - name: Setup Rust
```

- [ ] **Step 2: Validate YAML syntax**

```bash
python3 -c "import yaml; yaml.safe_load(open('.github/workflows/release.yml')); print('YAML valid')"
```
Expected: `YAML valid`

- [ ] **Step 3: Commit**

```bash
git add .github/workflows/release.yml
git commit -m "ci: add preflight check that config/cli-templates.toml exists

Produces a clear error instead of cryptic 'cp: cannot stat' if the
file is ever missing from the repository.

Co-authored-by: Copilot <223556219+Copilot@users.noreply.github.com>"
```

---

### Task 5: Modify Linux/macOS packaging step and add Windows packaging step

**Files:**
- Modify: `.github/workflows/release.yml` (packaging steps, ~lines 193-212)

Two changes in one task because they're co-located and both affect the "before upload" area:
1. Replace the existing "Create tarball" step with a staging-based version.
2. Add a new "Create zip (Windows)" step.

- [ ] **Step 1: Replace the "Create tarball" step**

```yaml
# BEFORE:
      - name: Create tarball (Linux and macOS)
        if: matrix.os != 'windows-latest'
        run: |
          cd target/${{ matrix.target }}/release
          tar czf ${{ matrix.asset_name }}.tar.gz ${{ matrix.artifact_name }}
          mv ${{ matrix.asset_name }}.tar.gz ../../../

# AFTER:
      - name: Create tarball (Linux and macOS)
        if: matrix.os != 'windows-latest'
        run: |
          mkdir -p staging/config
          cp target/${{ matrix.target }}/release/${{ matrix.artifact_name }} staging/
          cp config/cli-templates.toml staging/config/
          tar czf ${{ matrix.asset_name }}.tar.gz -C staging .
          rm -rf staging
```

- [ ] **Step 2: Add the "Create zip (Windows)" step immediately after the tarball step**

Insert this block after the "Create tarball" step and before the upload steps:

```yaml
      - name: Create zip (Windows)
        if: matrix.os == 'windows-latest'
        shell: pwsh
        run: |
          New-Item -ItemType Directory -Force -Path staging\config
          Copy-Item "target\${{ matrix.target }}\release\${{ matrix.artifact_name }}" -Destination staging\
          Copy-Item "config\cli-templates.toml" -Destination staging\config\
          Push-Location staging
          Compress-Archive -Path "${{ matrix.artifact_name }}", config -DestinationPath "..\${{ matrix.asset_name }}.zip"
          Pop-Location
          Remove-Item -Recurse -Force staging
```

`Push-Location staging` is required: without it, `Compress-Archive` would record paths as `staging/dispatch-agent.exe` instead of `dispatch-agent.exe` inside the zip.

- [ ] **Step 3: Validate YAML syntax**

```bash
python3 -c "import yaml; yaml.safe_load(open('.github/workflows/release.yml')); print('YAML valid')"
```
Expected: `YAML valid`

- [ ] **Step 4: Commit**

```bash
git add .github/workflows/release.yml
git commit -m "ci: rewrite packaging steps to bundle config/cli-templates.toml

Linux/macOS: stage binary + config/, tar from staging dir so entries
are ./binary and ./config/cli-templates.toml.

Windows: stage binary + config/, Push-Location into staging before
Compress-Archive so entries are at archive root (not staging/ prefixed).

Co-authored-by: Copilot <223556219+Copilot@users.noreply.github.com>"
```

---

### Task 6: Add archive verification steps

**Files:**
- Modify: `.github/workflows/release.yml` (after packaging steps, before upload steps)

These steps are the primary automated safeguard — every build job verifies archive contents before uploading.

- [ ] **Step 1: Add verification steps after the packaging steps (after "Create zip" step, before upload steps)**

```yaml
      - name: Verify archive contents (Linux and macOS)
        if: matrix.os != 'windows-latest'
        run: |
          tar tzf ${{ matrix.asset_name }}.tar.gz | grep -qxF './config/cli-templates.toml'
          tar tzf ${{ matrix.asset_name }}.tar.gz | grep -qxF './${{ matrix.artifact_name }}'

      - name: Verify archive contents (Windows)
        if: matrix.os == 'windows-latest'
        shell: pwsh
        run: |
          Expand-Archive -Path "${{ matrix.asset_name }}.zip" -DestinationPath verify_tmp -Force
          if (-not (Test-Path "verify_tmp\${{ matrix.artifact_name }}")) { Write-Error "Missing binary"; exit 1 }
          if (-not (Test-Path "verify_tmp\config\cli-templates.toml")) { Write-Error "Missing config"; exit 1 }
          Remove-Item -Recurse -Force verify_tmp
```

`grep -qxF` uses `-x` (match whole line) and `-F` (fixed string, not regex) for exact matching. The `./` prefix is expected because the archive is built with `tar czf -C staging .`.

- [ ] **Step 2: Validate YAML syntax**

```bash
python3 -c "import yaml; yaml.safe_load(open('.github/workflows/release.yml')); print('YAML valid')"
```
Expected: `YAML valid`

- [ ] **Step 3: Commit**

```bash
git add .github/workflows/release.yml
git commit -m "ci: add archive content verification steps to build job

Each build job now verifies that the packaged archive contains both
the binary and config/cli-templates.toml before uploading. This is the
primary safeguard against regressions; manual inspection is not needed.

Co-authored-by: Copilot <223556219+Copilot@users.noreply.github.com>"
```

---

### Task 7: Merge upload artifact steps into one

**Files:**
- Modify: `.github/workflows/release.yml` (upload steps, ~lines 200-212)

Replace the two platform-specific upload steps with a single step using `matrix.archive_ext`.

- [ ] **Step 1: Replace both upload steps**

```yaml
# BEFORE (two steps):
      - name: Upload artifacts (Linux and macOS)
        if: matrix.os != 'windows-latest'
        uses: actions/upload-artifact@v4
        with:
          name: ${{ matrix.asset_name }}
          path: ${{ matrix.asset_name }}.tar.gz

      - name: Upload artifacts (Windows)
        if: matrix.os == 'windows-latest'
        uses: actions/upload-artifact@v4
        with:
          name: ${{ matrix.asset_name }}
          path: target/${{ matrix.target }}/release/${{ matrix.artifact_name }}

# AFTER (one step):
      - name: Upload artifacts
        uses: actions/upload-artifact@v4
        with:
          name: ${{ matrix.asset_name }}
          path: ${{ matrix.asset_name }}.${{ matrix.archive_ext }}
```

- [ ] **Step 2: Validate YAML syntax**

```bash
python3 -c "import yaml; yaml.safe_load(open('.github/workflows/release.yml')); print('YAML valid')"
```
Expected: `YAML valid`

- [ ] **Step 3: Commit**

```bash
git add .github/workflows/release.yml
git commit -m "ci: merge upload artifact steps into one using matrix.archive_ext

Both platforms now produce a single archive file. The unified step
uses matrix.archive_ext (tar.gz or zip) set per matrix entry.

Co-authored-by: Copilot <223556219+Copilot@users.noreply.github.com>"
```

---

### Task 8: Update release job steps

**Files:**
- Modify: `.github/workflows/release.yml` (release job, ~lines 230-253)

Two changes: remove dead `*.exe` from "Display structure", and simplify "Prepare release files" to drop the Windows `.exe` special-case loop.

- [ ] **Step 1: Update "Display structure" step**

```yaml
# BEFORE:
      - name: Display structure
        run: |
          echo "Current directory structure:"
          ls -la
          echo "Artifacts directory:"
          ls -la artifacts/
          echo "Looking for artifacts:"
          find artifacts -type f \( -name "*.tar.gz" -o -name "*.zip" -o -name "*.exe" \)

# AFTER:
      - name: Display structure
        run: |
          echo "Current directory structure:"
          ls -la
          echo "Artifacts directory:"
          ls -la artifacts/
          echo "Looking for artifacts:"
          find artifacts -type f \( -name "*.tar.gz" -o -name "*.zip" \)
```

- [ ] **Step 2: Update "Prepare release files" step**

```yaml
# BEFORE:
      - name: Prepare release files
        run: |
          mkdir -p release_files
          find artifacts -type f -name "*.tar.gz" -exec cp {} release_files/ \;
          for dir in artifacts/*; do
            if [ -d "$dir" ] && [[ "$dir" == *"windows"* ]]; then
              asset_name=$(basename "$dir")
              exe_file="$dir/dispatch-agent.exe"
              if [ -f "$exe_file" ]; then
                cp "$exe_file" "release_files/$asset_name"
              fi
            fi
          done
          echo "Files in release_files:"
          ls -la release_files/

# AFTER:
      - name: Prepare release files
        run: |
          mkdir -p release_files
          find artifacts -type f \( -name "*.tar.gz" -o -name "*.zip" \) -exec cp {} release_files/ \;
          echo "Files in release_files:"
          ls -la release_files/
```

- [ ] **Step 3: Validate YAML syntax**

```bash
python3 -c "import yaml; yaml.safe_load(open('.github/workflows/release.yml')); print('YAML valid')"
```
Expected: `YAML valid`

- [ ] **Step 4: Commit**

```bash
git add .github/workflows/release.yml
git commit -m "ci: update release job steps for archive-only artifacts

- Display structure: remove dead *.exe search
- Prepare release files: drop Windows .exe special-case loop; all
  artifacts are now archives (*.tar.gz or *.zip)

Co-authored-by: Copilot <223556219+Copilot@users.noreply.github.com>"
```

---

### Task 9: Update release body template with installation instructions

**Files:**
- Modify: `.github/workflows/release.yml` (release body template, ~lines 291-308)

Adds installation instructions explaining the new archive format and the required directory structure (binary must be co-located with `config/`).

- [ ] **Step 1: Update the release body in the "Create Release" step**

```yaml
# BEFORE (body section):
          body: |
            ${{ steps.tag_message.outputs.message }}

            ---

            ## 📦 Downloads

            Please download the appropriate version for your system from below.

            ## 🔒 File Verification

            Use the SHA256SUMS file to verify the integrity of downloaded files.

            ---

            ## 📝 Auto-generated Changelog

# AFTER:
          body: |
            ${{ steps.tag_message.outputs.message }}

            ---

            ## 📦 Installation

            **Linux/macOS:**
            ```sh
            mkdir -p /opt/dispatch-agent
            tar xzf dispatch-agent-<platform>.tar.gz -C /opt/dispatch-agent
            ln -s /opt/dispatch-agent/dispatch-agent /usr/local/bin/dispatch-agent
            ```
            > The binary reads `config/cli-templates.toml` from the same directory it is installed in. Use a dedicated directory (e.g. `/opt/dispatch-agent`) rather than extracting directly into `/usr/local/bin`.

            **Windows:** Extract the `.zip` to a directory of your choice. Keep `dispatch-agent.exe` and the `config/` folder together in the same directory.

            ## 🔒 File Verification

            Use the SHA256SUMS file to verify the integrity of downloaded files.

            ---

            ## 📝 Auto-generated Changelog
```

- [ ] **Step 2: Validate YAML syntax**

```bash
python3 -c "import yaml; yaml.safe_load(open('.github/workflows/release.yml')); print('YAML valid')"
```
Expected: `YAML valid`

- [ ] **Step 3: Final end-to-end review of the complete workflow file**

Skim the full file and confirm:
- All 13 matrix entries have `archive_ext`
- Windows `asset_name` values have no `.exe` suffix
- `Verify config file exists` step is present after checkout
- `Create tarball` step uses staging directory
- `Create zip (Windows)` step is present with `Push-Location staging`
- Both verify steps are present before the upload step
- Single `Upload artifacts` step uses `${{ matrix.archive_ext }}`
- Release job has no `*.exe` search, no `for dir` loop
- Release body has installation instructions

```bash
grep -n "archive_ext\|asset_name\|\.exe\|staging\|Push-Location\|verify_tmp\|Verify archive" .github/workflows/release.yml
```

- [ ] **Step 4: Commit**

```bash
git add .github/workflows/release.yml
git commit -m "ci: add installation instructions to release body template

Explains the archive structure and the requirement to keep dispatch-agent
binary co-located with config/ directory. Recommends /opt/dispatch-agent
over /usr/local/bin to avoid placing config/ in a system directory.

Co-authored-by: Copilot <223556219+Copilot@users.noreply.github.com>"
```

---

## Post-Implementation Checklist

- [ ] All 9 task commits are on the branch / main
- [ ] `python3 -c "import yaml; yaml.safe_load(open('.github/workflows/release.yml')); print('YAML valid')"` passes on the final file
- [ ] **Optional manual test (first deploy only):** Temporarily set `draft: true` in "Create Release" step, manually create test tag (`git tag v0.0.0-test && git push origin v0.0.0-test`), trigger via `workflow_dispatch`, confirm all 13 build jobs pass their verify steps, then delete the draft release and tag
- [ ] **Out-of-band (do not forget):** Update the `gpinstall.ps1` Gist to download `.zip` and extract, coordinated with the first release using this format

