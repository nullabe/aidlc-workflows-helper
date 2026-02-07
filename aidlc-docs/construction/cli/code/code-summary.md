# Code Generation Summary — aidlc-workflows-helper

## Architecture
Single Rust binary with 10 focused modules, orchestrated from `main.rs`.

## Module Map

| Module | Purpose | Key Functions |
|--------|---------|---------------|
| `main.rs` | Entry point, orchestrates the full interactive flow | `run()` |
| `banner.rs` | ASCII art banner with version | `print_banner()` |
| `ui.rs` | Styled terminal output (checkmarks, errors, info) | `step_done()`, `error()`, `info()`, `warn()`, `header()` |
| `prompt.rs` | Interactive prompts (folder, overwrite, gitignore, commit) | `select_folder()`, `confirm_overwrite()`, `select_commit_workflow()` |
| `github.rs` | GitHub API — fetch latest release info | `fetch_latest_release()` |
| `download.rs` | HTTPS download + SHA-256 verification | `download_to()`, `verify_checksum()` |
| `cache.rs` | Version-keyed download cache | `cached_zip_path()`, `has_cached()`, `cleanup_old_versions()` |
| `extract.rs` | Zip extraction + file installation | `extract_and_install()`, `rules_exist()` |
| `patch.rs` | Patch core-workflow.md (paths + commit workflow) | `patch_rule_details_path()`, `patch_commit_workflow()` |
| `gitignore.rs` | .gitignore manipulation | `add_to_gitignore()` |
| `integrity.rs` | SHA-256 integrity manifest for installed rules | `write_manifest()`, `verify_manifest()` |

## Dependencies
- `reqwest` (blocking, rustls-tls, json) — HTTPS client
- `serde` + `serde_json` — JSON parsing
- `zip` — zip extraction
- `sha2` — SHA-256 checksums
- `dialoguer` — interactive prompts
- `console` — styled terminal output
- `indicatif` — spinners
- `dirs` — platform cache directory
- `walkdir` — file tree listing
- `anyhow` — error handling

## Quality Gates
- `cargo check` ✓
- `cargo clippy` ✓ (no warnings)
- `cargo fmt --check` ✓
