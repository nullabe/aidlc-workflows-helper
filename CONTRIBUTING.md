# Contributing to aidlc-workflows-helper

Thank you for your interest in contributing! This document explains how to get started, what we expect from contributions, and how the review process works.

## Getting Started

### Prerequisites

- [Rust](https://rustup.rs/) (edition 2021+)
- Git

### Setup

```bash
git clone https://github.com/awslabs/aidlc-workflows-helper.git
cd aidlc-workflows-helper
cargo build
cargo test
```

If both commands succeed, you're ready to go.

## How to Contribute

### Reporting Bugs

Open an issue with:
- A clear title describing the problem
- Steps to reproduce
- Expected vs actual behavior
- Your OS and Rust version (`rustc --version`)

### Suggesting Features

Open an issue with:
- A description of the feature and why it's useful
- How it fits with the existing tool (folder selection → download → install → patch → gitignore flow)
- Any alternatives you've considered

### Submitting Code

1. Fork the repository
2. Create a feature branch from `main`:
   ```bash
   git checkout -b feat/my-feature
   ```
3. Make your changes (see [Code Guidelines](#code-guidelines) below)
4. Ensure all quality gates pass:
   ```bash
   cargo test
   cargo clippy
   cargo fmt --check
   ```
5. Commit using [conventional commits](#commit-conventions)
6. Push and open a pull request against `main`

## Code Guidelines

### Project Structure

Each module has a single responsibility. If you're adding a new feature, consider whether it belongs in an existing module or warrants a new one.

```
src/
├── main.rs        # Orchestration — wires all modules together
├── banner.rs      # ASCII art banner display
├── ui.rs          # Styled terminal output helpers
├── prompt.rs      # Interactive user prompts (dialoguer)
├── github.rs      # GitHub Releases API client
├── download.rs    # HTTPS download + SHA-256 verification
├── cache.rs       # Version-keyed download cache
├── extract.rs     # Zip extraction + file installation
├── patch.rs       # core-workflow.md patching
├── gitignore.rs   # .gitignore manipulation
└── integrity.rs   # Integrity manifest (.aidlc-integrity.sha256)
```

### Style

- Follow standard Rust conventions (`cargo fmt` enforces formatting)
- All public functions must have doc comments (`///`)
- Keep functions focused — if a function does two things, split it
- Use `anyhow::Result` for error handling; provide context with `.context()`
- Prefer returning `Result` over panicking — no `unwrap()` in production code (tests are fine)

### Testing

- Add unit tests for any new logic in the same file under `#[cfg(test)]`
- Use `tempfile::tempdir()` for filesystem tests — never touch the real filesystem
- Tests should be self-contained and not depend on network access or external state
- Run `cargo test` before submitting — all tests must pass

### Security

This tool downloads and installs files from the internet, so security matters:

- All HTTP requests must use HTTPS only (enforced via `reqwest`'s `https_only`)
- Downloads are restricted to `https://github.com/awslabs/aidlc-workflows` — do not add other download sources without discussion
- File integrity is verified with SHA-256 checksums
- Never disable TLS certificate validation

## Commit Conventions

We use [Conventional Commits](https://www.conventionalcommits.org/):

| Prefix | Use for |
|--------|---------|
| `feat:` | New features or capabilities |
| `fix:` | Bug fixes |
| `docs:` | Documentation changes |
| `test:` | Adding or updating tests |
| `refactor:` | Code changes that don't add features or fix bugs |
| `ci:` | CI/CD workflow changes |
| `chore:` | Maintenance tasks (dependency updates, config changes) |

Examples:
```
feat: add support for .windsurf/rules folder preset
fix: handle missing .gitignore gracefully on Windows
docs: add troubleshooting section to README
test: add edge case tests for gitignore deduplication
refactor: extract HTTP client setup into shared helper
```

Keep commits small and focused — one concern per commit.

## Pull Request Process

1. Ensure your branch is up to date with `main`
2. All quality gates must pass (`cargo test`, `cargo clippy`, `cargo fmt --check`)
3. Provide a clear PR description explaining what changed and why
4. Link any related issues
5. A maintainer will review your PR — expect feedback and be open to changes

## Code of Conduct

This project follows the [Amazon Open Source Code of Conduct](https://aws.github.io/code-of-conduct). Please be respectful and constructive in all interactions.

## Security Issue Notifications

If you discover a potential security issue, please do **not** create a public GitHub issue. Instead, follow the instructions in [SECURITY.md](SECURITY.md) or email the AWS security team directly.

## License

By contributing, you agree that your contributions will be licensed under the [MIT-0 License](LICENSE).
