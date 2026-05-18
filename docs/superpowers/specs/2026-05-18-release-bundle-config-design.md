# Design: Bundle config/ into Release Archives

## Problem

The current `release.yml` workflow:
- Packages Linux/macOS binaries into `.tar.gz` containing **only** the binary
- Uploads Windows `.exe` directly with no archive at all
- Does **not** include `config/cli-templates.toml` in any release artifact

Users who download a release must manually locate and place `config/cli-templates.toml` themselves, which is error-prone and unintuitive.

## Goal

Bundle `config/cli-templates.toml` (preserving the `config/` subdirectory) together with the binary in every release archive. Windows gets `.zip`; Linux/macOS keep `.tar.gz`.

## Breaking Changes

**Windows:** Artifact format changes from bare `.exe` to `.zip`. Any automation that downloads Windows releases by filename (e.g., `*-windows-*.exe`) must be updated to target `*-windows-*.zip` and extract before use. Note explicitly in release notes.

**Linux/macOS:** The `.tar.gz` archives previously contained only the binary. After this change they also contain `config/`. Any script that extracts a specific file (`tar xzf ... dispatch-agent`) or uses `--strip-components` to handle a top-level directory (there is none, but scripts assuming a single-file archive may behave unexpectedly) may need updating. Note this content change in release notes.

## Archive Structure (all platforms)

```
<archive-root>/
├── dispatch-agent        (or dispatch-agent.exe on Windows)
└── config/
    └── cli-templates.toml
```

**No top-level directory.** Files are placed directly at the archive root (not inside a `dispatch-agent-vX.Y.Z/` subdirectory). This is an intentional choice for ease of use: users can extract to any target directory without needing to strip a prefix. The trade-off is that extracting into the current directory without a target path will place files inline — users should always specify a destination.

**Bundled config scope.** Only `config/cli-templates.toml` is explicitly copied into the archive. The implementation uses `mkdir -p staging/config && cp config/cli-templates.toml staging/config/` (not `cp -r config`) to avoid accidentally including future files added to `config/` that are not intended for distribution.

## Changes Required

### 1. Matrix — Windows `asset_name` cleanup + add `archive_ext`

Remove the `.exe` suffix from `asset_name` for all Windows targets, and add an `archive_ext` field to **every one of the 13 matrix entries** (10 Linux/macOS + 3 Windows). Section 5's unified upload step uses `matrix.archive_ext`; any entry missing this field will cause that build job to fail.

| Target | `asset_name` (before) | `asset_name` (after) | `archive_ext` |
|--------|-----------------------|----------------------|---------------|
| `x86_64-pc-windows-msvc` | `dispatch-agent-windows-x86_64.exe` | `dispatch-agent-windows-x86_64` | `zip` |
| `i686-pc-windows-msvc` | `dispatch-agent-windows-i686.exe` | `dispatch-agent-windows-i686` | `zip` |
| `aarch64-pc-windows-msvc` | `dispatch-agent-windows-aarch64.exe` | `dispatch-agent-windows-aarch64` | `zip` |
| All 10 Linux/macOS targets | (unchanged) | (unchanged) | `tar.gz` |

`artifact_name` (the actual compiled binary filename) remains `dispatch-agent.exe` for Windows, `dispatch-agent` for others.

### 2. Build Job — Preflight: Verify config file exists

Add early in the build job (before any packaging step) to produce a clear error if the config file is missing, rather than a cryptic `cp: cannot stat` failure:

```yaml
- name: Verify config file exists
  run: test -f config/cli-templates.toml
```

On Windows, add the equivalent:

```yaml
- name: Verify config file exists (Windows)
  if: matrix.os == 'windows-latest'
  shell: pwsh
  run: |
    if (-not (Test-Path "config\cli-templates.toml")) { Write-Error "config\cli-templates.toml not found"; exit 1 }
```

### 3. Build Job — Linux/macOS: Modify "Create tarball" step
Replace the current single-directory `tar` invocation with a staging-directory approach that merges the binary and the explicit config file:

```yaml
- name: Create tarball (Linux and macOS)
  if: matrix.os != 'windows-latest'
  run: |
    mkdir -p staging/config
    cp target/${{ matrix.target }}/release/${{ matrix.artifact_name }} staging/
    cp config/cli-templates.toml staging/config/
    tar czf ${{ matrix.asset_name }}.tar.gz -C staging .
    rm -rf staging
```

The checkout step already provides `config/cli-templates.toml` at the repo root. `cp` preserves the execute bit from `target/release/`, so `chmod +x` is not needed.

### 4. Build Job — Windows: New "Create zip" step

Add a new step after the strip steps (Windows skips stripping anyway) to produce a `.zip`, using explicit file paths (not glob) to prevent unintended files from being included:

```yaml
- name: Create zip (Windows)
  if: matrix.os == 'windows-latest'
  shell: pwsh
  run: |
    New-Item -ItemType Directory -Force -Path staging\config
    Copy-Item "target\${{ matrix.target }}\release\${{ matrix.artifact_name }}" -Destination staging\
    Copy-Item "config\cli-templates.toml" -Destination staging\config\
    Compress-Archive -Path staging\${{ matrix.artifact_name }}, staging\config -DestinationPath "${{ matrix.asset_name }}.zip"
    Remove-Item -Recurse -Force staging
```

Using explicit `-Path staging\<binary>, staging\config` instead of `staging\*` prevents accidentally bundling hidden files or other workspace artifacts.

### 5. Build Job — Add Archive Verification Steps

After packaging, verify the archive contains the expected files before uploading. This is the primary automated safeguard against regressions:

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

### 6. Build Job — Merge Upload Artifacts Steps

Both platforms now upload a single archive file. The two separate upload steps can be collapsed into one, using `matrix.archive_ext` (defined in Section 1):

```yaml
- name: Upload artifacts
  uses: actions/upload-artifact@v4
  with:
    name: ${{ matrix.asset_name }}
    path: ${{ matrix.asset_name }}.${{ matrix.archive_ext }}
```

### 7. Release Job — Simplify "Prepare release files" step

All artifacts are now archives. Remove the entire `for dir` loop that handled the Windows `.exe` special case. Only archive files need to be collected:

```yaml
- name: Prepare release files
  run: |
    mkdir -p release_files
    find artifacts -type f \( -name "*.tar.gz" -o -name "*.zip" \) -exec cp {} release_files/ \;
    echo "Files in release_files:"
    ls -la release_files/
```

### 8. Release Job — Update "Display structure" debug step

The current step searches for `*.exe` in addition to archives. Since no bare `.exe` files will exist post-change, update to only search for archives:

```yaml
- name: Display structure
  run: |
    echo "Current directory structure:"
    ls -la
    echo "Artifacts directory:"
    ls -la artifacts/
    echo "Looking for artifacts:"
    find artifacts -type f \( -name "*.tar.gz" -o -name "*.zip" \)
```

### 9. SHA256SUMS Step — No Changes Required

The `sha256sum *` command operates on all files present in `release_files/`. Before this change, that included `.tar.gz` files and bare `.exe` files. After this change, it will include `.tar.gz` and `.zip` files. The step logic is unchanged and checksums will correctly cover all artifacts.

### 10. Add `.gitattributes` for LF line endings

Windows `actions/checkout` may convert LF → CRLF for text files, causing `config/cli-templates.toml` to have different line endings between Windows `.zip` and Linux/macOS `.tar.gz`. This would produce different SHA256 checksums for the same logical file across platforms, and could confuse cross-platform users.

Add a `.gitattributes` entry to force LF in the config file regardless of platform:

```
config/cli-templates.toml text eol=lf
```

This is a repo-level change that applies to all contributors and all CI runners.

## Out-of-Band Actions Required

### Update `gpinstall.ps1` (external Gist)

The README references an installer script hosted at:
```
https://gist.githubusercontent.com/superyngo/a6b786af38b8b4c2ce15a70ae5387bd7/raw/gpinstall.ps1
```

This script likely downloads a `.exe` directly. After this change ships, the Gist must be updated to:
1. Download `dispatch-agent-windows-<arch>.zip` instead of `.exe`
2. Extract the archive to the installation directory (preserving `config/` subdirectory)

The README command (`irm ... gpinstall.ps1 | iex`) itself does not need to change — only the Gist content. This update must be coordinated with the first release using the new format.

### Update `release.yml` release body template

The release body template (lines 291–306 of `release.yml`) currently has no extraction instructions. Consider adding a brief extraction example to assist users who download manually:

```
## 📦 Extraction

**Linux/macOS:** `tar xzf dispatch-agent-<platform>.tar.gz -C /usr/local/bin`
**Windows:** Extract the `.zip` and place both `dispatch-agent.exe` and the `config/` folder in the same directory.
```

## Future Config Files

If additional files need to be bundled in future releases, both packaging steps must be updated:
- Linux/macOS: add `cp <file> staging/<dest>` before the `tar` command
- Windows: add `Copy-Item "<file>" -Destination staging\<dest>` and add the path to the `Compress-Archive -Path` argument list

There is no single-point config for "files to bundle" — maintainers must keep the two steps in sync manually.

## Non-Changes

- Archive naming for Linux/macOS stays unchanged (e.g., `dispatch-agent-linux-x86_64.tar.gz`)
- All build, cross-compilation, and strip steps remain unchanged
- The `Generate checksums` step (`sha256sum *`) requires no modification — see Section 9

## Verification

**The automated archive verification steps added in Section 4 are the primary safeguard.** Every build job verifies archive contents before uploading — manual inspection is not required for routine releases.

**If a manual workflow test is needed** (e.g., for a first-time deploy of this change):
1. Temporarily set `draft: true` in the `Create Release` step to prevent a public release
2. Create the test tag manually first (`git tag v0.0.0-test && git push origin v0.0.0-test`), since the release job's checkout uses `ref: ${{ github.event.inputs.version || github.ref }}` which requires the tag to exist
3. Trigger via `workflow_dispatch` with that tag name
4. Confirm all 13 build jobs pass their verify steps
5. Delete the draft release and test tag before the real release

## Success Criteria

- Every release artifact (`.tar.gz` and `.zip`) extracts to a directory containing both the binary and `config/cli-templates.toml`
- Each build job's verify step passes (automated, not manual)
- SHA256SUMS covers all archives
- No bare `.exe` files appear in the release
- `config/cli-templates.toml` uses LF line endings in all archives (enforced by `.gitattributes`)
- Release notes for the first release using this format include the breaking change notice for Windows artifact format
- `gpinstall.ps1` Gist updated before or alongside the first release using this format

## Future Considerations (Non-Blocking)

- **Reproducible archives:** `Compress-Archive` embeds the current timestamp in each zip entry; `tar` embeds UID/GID and mtime. Neither affects release functionality. If build reproducibility becomes a requirement, add `--sort=name --mtime='1970-01-01 00:00:00' --owner=0 --group=0 --numeric-owner` to the `tar` command, and replace `Compress-Archive` with a .NET `ZipFile` call using a fixed `DateTimeOffset`.
