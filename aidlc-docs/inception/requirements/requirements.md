# Requirements — aidlc-workflows-helper

## Intent Analysis
- **User Request**: Build a Rust CLI tool that installs AI-DLC workflow rules into any project
- **Request Type**: New Project (greenfield)
- **Scope**: Single component (one CLI binary)
- **Complexity**: Moderate — HTTP download, zip extraction, file patching, interactive prompts

## Functional Requirements

### FR-1: Interactive Folder Selection
- On launch, prompt the user to enter the target folder path where rules should be installed (e.g. `.kiro`, `.amazonq`, `.cursor/rules`, or any custom path).
- The folder is relative to the current working directory.

### FR-2: Patch core-workflow.md References
- After copying files, scan `core-workflow.md` inside the installed rules and replace the default rule-details path (`.aiassistant/aws-aidlc-rule-details/`, `.kiro/aws-aidlc-rule-details/`, `.amazonq/aws-aidlc-rule-details/`) with the actual path the user chose.
- This ensures the rules reference the correct location regardless of the folder name.

### FR-3: Gitignore Prompt (Rules Folder)
- Ask the user (y/n) whether to add the chosen rules folder to `.gitignore`.
- If yes, append the folder path to `.gitignore` (create the file if it doesn't exist). Do not duplicate entries.

### FR-3b: Gitignore Prompt (aidlc-docs)
- Ask the user (y/n) whether to add `aidlc-docs/` to `.gitignore`.
- Same behavior: append if yes, no duplicates, create file if needed.

### FR-4: Download Latest Release
- Fetch the latest release zip from `https://github.com/awslabs/aidlc-workflows/releases/latest` via the GitHub Releases API.
- No version selection — always latest.

### FR-5: Extract and Install Rules
- Extract the zip to a temp directory.
- Locate the `aidlc-rules/` folder inside the extracted content.
- Copy `aws-aidlc-rules/` into `<chosen-folder>/rules/` (or equivalent sub-path).
- Copy `aws-aidlc-rule-details/` into `<chosen-folder>/`.
- The final structure mirrors the README instructions but adapted to the user's chosen folder.

### FR-6: Overwrite Handling
- If the target rules folder already exists, ask the user whether to overwrite or skip.

### FR-7: Polished Console UI
- Use spinners/loaders during download and extraction steps.
- Use colored/styled output, checkmarks for completed steps, clear section headers.
- The overall experience should feel polished and professional (not just plain println).

### FR-8: Download Cache
- Cache the downloaded release zip in a platform-appropriate cache directory (e.g. `~/.cache/aidlc-workflows-helper/` on Linux/macOS, `%LOCALAPPDATA%\aidlc-workflows-helper\cache\` on Windows).
- Cache is keyed by release version tag (e.g. `v0.1.1/`).
- On subsequent runs, if the cached version matches the latest release, skip the download and use the cached zip.
- If a new version is detected, delete the old cached version and download the new one (transparent cleanup).
- Display a message when using cache ("Using cached release v0.1.1") vs downloading fresh.

### FR-9: Security — Checksum Verification
- After downloading the zip, verify its SHA-256 checksum against the checksum published in the GitHub release assets (if available).
- If no checksum asset is published, compute and store the checksum on first download, then verify on subsequent runs from cache.
- If checksum verification fails, abort with a clear error and delete the corrupted file.

### FR-10: Security — Integrity Verification of Installed Rules
- After copying rule files to the target folder, compute SHA-256 hashes of all installed `.md` files and store them in a manifest file (`<folder>/.aidlc-integrity.sha256`).
- On subsequent runs (when overwrite is chosen), warn the user if any existing rule files have been modified since installation by comparing against the stored manifest.
- This protects against unintended or malicious tampering of the rule files.

### FR-11: Security — Secure Transport and Trusted Source
- All HTTP requests must use HTTPS only.
- Only download from `https://github.com/awslabs/aidlc-workflows` — reject any redirects to non-GitHub domains.
- Validate TLS certificates (no `--insecure` equivalent).

### FR-12: Automatic Gitignore for audit.md
- Always add `aidlc-docs/audit.md` to `.gitignore` — no prompt, unconditional.
- This is done regardless of the user's answers to the other gitignore prompts.
- Same no-duplicate / create-if-missing behavior as FR-3.

### FR-13: Commit Workflow Preference Prompt
- Ask the user about their preferred commit workflow (e.g. "Conventional Commits", "Free-form", "None / I handle commits myself").
- If the user selects a commit style, patch `core-workflow.md` to include a MANDATORY commit section matching their preference (conventional commits format, frequency rules, etc.).
- If the user selects "None", do not add any commit rules to `core-workflow.md`.

## Non-Functional Requirements

### NFR-1: Single Static Binary
- Produce a single standalone executable with no runtime dependencies.
- Use `lto = true`, `strip = true`, `opt-level = "z"` in release profile.

### NFR-2: Cross-Platform Builds (GitHub Actions)
- Build targets: macOS x86_64, macOS aarch64, Linux x86_64, Linux aarch64, Windows x86_64.
- Use GitHub Actions with a release workflow triggered on tags.

### NFR-3: Open-Source Documentation
- README.md with installation instructions (curl one-liner), usage, and contributing guide.
- MIT-0 license (matching upstream project).

### NFR-4: Conventional Commits
- All commits follow conventional commit format (`feat:`, `fix:`, `docs:`, `ci:`, etc.).

### NFR-5: Unit Testing and Linting
- Unit tests for core logic (path patching, gitignore manipulation, zip extraction).
- `clippy` linting with no warnings in CI.
- `rustfmt` formatting enforced in CI.

### NFR-6: Developer Experience (DX)
- Clean project structure with separated modules.
- Well-documented public functions.
- Easy `cargo build` / `cargo test` / `cargo clippy` workflow.

## Out of Scope
- Non-interactive / CI flag mode (interactive only)
- Version selection (always latest)
- Auto-update mechanism
