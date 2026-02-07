# Code Generation Plan — aidlc-workflows-helper (Single Unit)

## Unit Context
- **Unit**: Single CLI binary (`aidlc-workflows-helper`)
- **Type**: Greenfield Rust CLI
- **Stories**: US-1 through US-11
- **Code Location**: Workspace root (`src/`, `Cargo.toml`)
- **Documentation**: `aidlc-docs/construction/cli/code/`

## Dependencies (Crates)
- `reqwest` (blocking, rustls-tls) — HTTPS downloads
- `serde` + `serde_json` — GitHub API JSON parsing
- `zip` — zip extraction
- `sha2` — SHA-256 checksums
- `dialoguer` — interactive prompts (Select, Confirm, Input)
- `console` — colored/styled terminal output (comes with dialoguer)
- `indicatif` — spinners
- `dirs` — platform-appropriate cache directory
- `walkdir` — recursive file tree listing
- `anyhow` — error handling

## Module Structure
```
src/
├── main.rs          — entry point, orchestrates the flow
├── banner.rs        — ASCII art banner display
├── prompt.rs        — folder selection, gitignore prompts, commit workflow prompt
├── github.rs        — GitHub API: fetch latest release info
├── download.rs      — download zip, checksum verification
├── cache.rs         — cache management (store, lookup, cleanup)
├── extract.rs       — zip extraction + file copying to target
├── patch.rs         — patch core-workflow.md path references + commit workflow section
├── gitignore.rs     — .gitignore manipulation (append, no-duplicate, create)
├── integrity.rs     — .aidlc-integrity.sha256 manifest (write + verify)
└── ui.rs            — spinners, checkmarks, styled output helpers
```

## Execution Steps

### Step 1: Project Setup — Cargo.toml + Module Scaffold
- [x] Update `Cargo.toml` with all dependencies and release profile (lto, strip, opt-level)
- [x] Create all module files with `mod` declarations in `main.rs`
- [ ] Stories: NFR-1, NFR-6
- **Commit**: `feat: add dependencies and module scaffold`

### Step 2: UI Helpers + Banner
- [x] Implement `ui.rs` — `success()`, `error()`, `info()`, `step_done()` styled output helpers
- [x] Implement `banner.rs` — ASCII art banner with version from `env!("CARGO_PKG_VERSION")`
- [ ] Stories: US-1 (AC-1.1), US-6 (AC-6.2, AC-6.3, AC-6.4)
- **Commit**: `feat: add UI helpers and ASCII art banner`

### Step 3: Interactive Prompts
- [x] Implement `prompt.rs` — `select_folder()` with presets (.kiro/steering, .amazonq/rules, .cursor/rules, custom)
- [x] Implement `prompt.rs` — `confirm_overwrite()`, `confirm_gitignore()`, `confirm_gitignore_aidlc_docs()`
- [x] Implement `prompt.rs` — `select_commit_workflow()` (Conventional Commits / Free-form / None)
- [ ] Stories: US-1 (AC-1.2), US-2 (AC-2.1), US-3 (AC-3.1), US-4 (AC-4.1), US-11 (AC-11.1)
- **Commit**: `feat: add interactive prompts (folder, overwrite, gitignore, commit workflow)`

### Step 4: GitHub API + Download
- [x] Implement `github.rs` — `fetch_latest_release()` → returns tag name + zip URL + optional checksum URL
- [x] Implement `download.rs` — `download_zip()` with spinner, HTTPS-only, TLS validation
- [x] Implement `download.rs` — `verify_checksum()` SHA-256 verification
- [ ] Stories: US-1 (AC-1.3), US-5 (AC-5.1, AC-5.2, AC-5.3), US-8 (AC-8.1, AC-8.2, AC-8.3), US-10 (AC-10.1, AC-10.2, AC-10.3)
- **Commit**: `feat: add GitHub API client and secure download with checksum`

### Step 5: Cache Management
- [x] Implement `cache.rs` — `get_cache_dir()`, `get_cached_zip()`, `store_in_cache()`, `cleanup_old_versions()`
- [x] Integrate cache into download flow (skip download if cached version matches latest)
- [ ] Stories: US-7 (AC-7.1, AC-7.2, AC-7.3, AC-7.4)
- **Commit**: `feat: add download cache with version-keyed storage`

### Step 6: Zip Extraction + File Installation
- [x] Implement `extract.rs` — `extract_and_install()` extracts zip, locates `aidlc-rules/`, copies `aws-aidlc-rules/` → `<folder>/rules/` and `aws-aidlc-rule-details/` → `<folder>/`
- [x] Handle the nested directory inside the zip (GitHub zips have a root folder like `aidlc-workflows-vX.Y.Z/`)
- [ ] Stories: US-1 (AC-1.4), US-2 (AC-2.3)
- **Commit**: `feat: add zip extraction and rules installation`

### Step 7: Path Patching + Commit Workflow Patching
- [x] Implement `patch.rs` — `patch_rule_details_path()` replaces default paths in `core-workflow.md` with user's chosen folder
- [x] Implement `patch.rs` — `patch_commit_workflow()` appends commit section based on user preference
- [ ] Stories: US-1 (AC-1.5), US-11 (AC-11.2, AC-11.3, AC-11.4), FR-2, FR-13
- **Commit**: `feat: add core-workflow.md patching (paths + commit workflow)`

### Step 8: Gitignore Management
- [x] Implement `gitignore.rs` — `add_to_gitignore()` with no-duplicate, create-if-missing
- [x] Handle FR-12: always add `aidlc-docs/audit.md` unconditionally
- [ ] Stories: US-3 (AC-3.1, AC-3.2, AC-3.3), US-4 (AC-4.1, AC-4.2), FR-12
- **Commit**: `feat: add gitignore management`

### Step 9: Integrity Manifest
- [x] Implement `integrity.rs` — `write_manifest()` computes SHA-256 of all installed `.md` files, writes `.aidlc-integrity.sha256`
- [x] Implement `integrity.rs` — `verify_manifest()` checks existing files against manifest, returns list of modified files
- [ ] Stories: US-9 (AC-9.1, AC-9.2, AC-9.3)
- **Commit**: `feat: add installed rules integrity manifest`

### Step 10: Main Orchestration + Final Summary
- [x] Wire everything together in `main.rs` — the full interactive flow
- [x] Implement final success summary with installed file tree (US-1 AC-1.7, US-6 AC-6.5)
- [x] Handle overwrite flow with integrity check warning
- [ ] Stories: US-1 (AC-1.6, AC-1.7), US-2 (AC-2.2), US-6 (AC-6.5)
- **Commit**: `feat: wire main orchestration and success summary`

### Step 11: Documentation + CI
- [x] Create `README.md` with installation (curl one-liner), usage, contributing guide
- [x] Create `LICENSE` (MIT-0)
- [x] Create `.github/workflows/release.yml` — cross-platform release builds on tag push (5 targets)
- [ ] Stories: NFR-2, NFR-3
- **Commit**: `docs: add README, LICENSE, and release CI workflow`

### Step 12: Code Generation Summary
- [x] Create `aidlc-docs/construction/cli/code/code-summary.md` with file listing and architecture overview

## Total: 12 steps, 11 commits
