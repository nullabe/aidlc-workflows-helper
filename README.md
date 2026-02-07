# aidlc-workflows-helper

CLI tool to install [AI-DLC workflow rules](https://github.com/awslabs/aidlc-workflows) into any project with a single command.

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
Download `aidlc-workflows-helper-x86_64-pc-windows-msvc.zip` from the [latest release](https://github.com/awslabs/aidlc-workflows-helper/releases/latest) and add it to your PATH.

## Usage

```bash
cd your-project
aidlc-workflows-helper
```

The tool will:
1. Ask you to choose a target folder (`.kiro/steering`, `.amazonq/rules`, `.cursor/rules`, or custom)
2. Download the latest AI-DLC rules from GitHub
3. Extract and install the rules into your chosen folder
4. Patch `core-workflow.md` so path references match your setup
5. Ask about your commit workflow preference
6. Optionally add folders to `.gitignore`

### Features

- **Folder presets** for Kiro, Amazon Q, Cursor, or any custom path
- **Download cache** — subsequent installs reuse the cached release
- **Checksum verification** — SHA-256 integrity check on downloads
- **Integrity manifest** — detects if installed rule files have been modified
- **Secure transport** — HTTPS-only, TLS validated, trusted source only
- **Commit workflow** — optionally adds conventional commit rules to `core-workflow.md`

## Building from Source

```bash
git clone https://github.com/awslabs/aidlc-workflows-helper.git
cd aidlc-workflows-helper
cargo build --release
```

The binary will be at `target/release/aidlc-workflows-helper`.

## Development

```bash
cargo check      # type-check
cargo clippy      # lint
cargo fmt         # format
cargo test        # run tests
```

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feat/my-feature`)
3. Commit using [conventional commits](https://www.conventionalcommits.org/) (`feat:`, `fix:`, `docs:`, etc.)
4. Open a pull request

## License

This project is licensed under the MIT-0 License. See [LICENSE](LICENSE) for details.
