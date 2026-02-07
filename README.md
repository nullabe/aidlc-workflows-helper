# aidlc-workflows-helper

CLI tool to install [AI-DLC (AI-Driven Development Life Cycle)](https://github.com/awslabs/aidlc-workflows) workflow rules into any project with a single command.

Instead of manually downloading, extracting, and copying rule files — just run `aidlc-workflows-helper` and answer a few prompts.

## What is AI-DLC?

[AI-DLC](https://aws.amazon.com/blogs/devops/ai-driven-development-life-cycle/) is a methodology that reimagines software development by positioning AI as a central collaborator rather than just an assistant. It guides AI coding agents through a structured three-phase workflow:

- **Inception** — AI transforms your intent into detailed requirements, user stories, and units of work. The team validates AI's proposals through structured questions.
- **Construction** — Using the validated context from Inception, AI proposes architecture, generates code, and produces tests. The team provides oversight on technical decisions.
- **Operations** — AI applies accumulated context to manage infrastructure and deployments, with team oversight.

Each phase builds on the previous one, giving AI increasingly rich context to work with. The methodology works with any agent that supports project-level rules: **[Kiro](https://kiro.dev/)**, **[Amazon Q Developer](https://docs.aws.amazon.com/amazonq/latest/qdeveloper-ug/q-in-IDE.html)**, **[Cursor](https://www.cursor.com/)**, and others.

The key principle: **AI proposes, humans approve.** Every critical decision requires explicit user confirmation.

This tool automates the setup described in the [AI-DLC workflows README](https://github.com/awslabs/aidlc-workflows#readme).

## Quick Install

### macOS (Apple Silicon)

```bash
curl -sL https://github.com/awslabs/aidlc-workflows-helper/releases/latest/download/aidlc-workflows-helper-aarch64-apple-darwin.tar.gz | tar xz
sudo mv aidlc-workflows-helper /usr/local/bin/
```

### macOS (Intel)

```bash
curl -sL https://github.com/awslabs/aidlc-workflows-helper/releases/latest/download/aidlc-workflows-helper-x86_64-apple-darwin.tar.gz | tar xz
sudo mv aidlc-workflows-helper /usr/local/bin/
```

### Linux (x86_64)

```bash
curl -sL https://github.com/awslabs/aidlc-workflows-helper/releases/latest/download/aidlc-workflows-helper-x86_64-unknown-linux-gnu.tar.gz | tar xz
sudo mv aidlc-workflows-helper /usr/local/bin/
```

### Linux (aarch64)

```bash
curl -sL https://github.com/awslabs/aidlc-workflows-helper/releases/latest/download/aidlc-workflows-helper-aarch64-unknown-linux-gnu.tar.gz | tar xz
sudo mv aidlc-workflows-helper /usr/local/bin/
```

### Windows (x86_64)

Download `aidlc-workflows-helper-x86_64-pc-windows-msvc.zip` from the [latest release](https://github.com/awslabs/aidlc-workflows-helper/releases/latest), extract it, and add the folder to your `PATH`.

## Usage

Navigate to your project root and run:

```bash
cd your-project
aidlc-workflows-helper
```

### What happens

1. **Folder selection** — Choose where rules should be installed:
   - `.kiro/steering` — for [Kiro IDE](https://kiro.dev/) / [Kiro CLI](https://kiro.dev/cli/)
   - `.amazonq/rules` — for [Amazon Q Developer](https://docs.aws.amazon.com/amazonq/latest/qdeveloper-ug/q-in-IDE.html)
   - `.cursor/rules` — for Cursor AI
   - Custom path — any folder you want

2. **Download** — Fetches the latest AI-DLC rules release from GitHub (HTTPS-only, checksum verified).

3. **Install** — Extracts and copies the rules into your chosen folder:
   ```
   <parent-folder>/
   ├── <rules-subfolder>/
   │   └── rules/
   │       └── core-workflow.md
   └── aws-aidlc-rule-details/
       ├── common/
       ├── construction/
       ├── inception/
       └── operations/
   ```
   For example, choosing `.kiro/steering` installs to `.kiro/steering/rules/` and `.kiro/aws-aidlc-rule-details/`.

4. **Patch** — Updates path references inside `core-workflow.md` to match your folder structure.

5. **Commit workflow** — Optionally adds commit convention rules (Conventional Commits, free-form, or none) to `core-workflow.md`.

6. **Gitignore** — Optionally adds the rules folder and `aidlc-docs/` to `.gitignore`. Always adds `aidlc-docs/audit.md` (contains session-specific data).

### After installation

Start any AI-DLC workflow by telling your AI agent:

> "Using AI-DLC, I want to build ..."

The agent will pick up the rules from the installed folder and guide you through the structured development workflow. For more details on how to use AI-DLC once installed, see the [AI-DLC workflows documentation](https://github.com/awslabs/aidlc-workflows#usage).

## Features

| Feature | Description |
|---------|-------------|
| **Folder presets** | One-click setup for Kiro, Amazon Q, Cursor, or any custom path |
| **Download cache** | Cached in `~/.cache/aidlc-workflows-helper/` — subsequent installs in other projects are instant |
| **Checksum verification** | SHA-256 integrity check on every download |
| **Integrity manifest** | Detects if installed rule files have been modified since installation |
| **Secure transport** | HTTPS-only, TLS validated, downloads only from `github.com/awslabs/aidlc-workflows` |
| **Commit workflow** | Optionally patches `core-workflow.md` with your team's commit conventions |
| **Overwrite protection** | Warns before overwriting existing rules, flags tampered files |

## Security

- All downloads use **HTTPS with TLS certificate validation** (via `rustls`).
- Downloads are restricted to `https://github.com/awslabs/aidlc-workflows` — redirects to other domains are rejected.
- Downloaded zips are verified with **SHA-256 checksums**.
- Installed rule files are tracked with an **integrity manifest** (`.aidlc-integrity.sha256`). On re-install, the tool warns if any files have been modified since the last installation.

## Building from Source

Requires [Rust](https://rustup.rs/) 1.85+ (edition 2024).

```bash
git clone https://github.com/awslabs/aidlc-workflows-helper.git
cd aidlc-workflows-helper
cargo build --release
```

The binary will be at `target/release/aidlc-workflows-helper`.

## Development

```bash
cargo check       # type-check
cargo clippy       # lint (must pass with 0 warnings)
cargo fmt          # format
cargo fmt --check  # verify formatting
cargo test         # run unit tests
```

### Project structure

```
src/
├── main.rs        # Entry point — orchestrates the full interactive flow
├── banner.rs      # ASCII art banner
├── ui.rs          # Styled terminal output (✓ ✗ ℹ ⚠)
├── prompt.rs      # Interactive prompts (folder, overwrite, gitignore, commit)
├── github.rs      # GitHub API — fetch latest release info
├── download.rs    # HTTPS download + SHA-256 verification
├── cache.rs       # Version-keyed download cache
├── extract.rs     # Zip extraction + file installation
├── patch.rs       # Patch core-workflow.md (paths + commit workflow)
├── gitignore.rs   # .gitignore manipulation
└── integrity.rs   # Integrity manifest for installed rules
```

### Quality gates

All of these must pass before merging:

- `cargo test` — all unit tests pass
- `cargo clippy` — zero warnings
- `cargo fmt --check` — consistent formatting

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines on how to contribute to this project.

## License

This project is licensed under the MIT-0 License. See [LICENSE](LICENSE) for details.
