# User Stories

All stories reference **Persona: Alex — The AI-DLC Adopter**.

---

## US-1: First-Run Happy Path

**As** Alex,
**I want** to run `aidlc-workflows-helper` in my project root and have it guide me through installing AI-DLC rules,
**So that** I can start using AI-DLC without manually downloading and copying files.

### Acceptance Criteria
- [ ] AC-1.1: On launch, an ASCII art banner with the tool name and version is displayed.
- [ ] AC-1.2: A list of common folder presets is shown (`.kiro`, `.amazonq`, `.cursor/rules`, custom path) with a short explanation of each.
- [ ] AC-1.3: After selection, the tool downloads the latest release zip from GitHub with a visible spinner.
- [ ] AC-1.4: The zip is extracted and `aws-aidlc-rules/` is copied into `<folder>/rules/` and `aws-aidlc-rule-details/` into `<folder>/`.
- [ ] AC-1.5: `core-workflow.md` inside the installed rules is patched so internal path references match the chosen folder.
- [ ] AC-1.6: Each completed step shows a green checkmark (✓) with a description.
- [ ] AC-1.7: A final success summary is displayed with the installed file tree.

---

## US-2: Overwrite Existing Rules

**As** Alex,
**I want** to be warned if rules already exist in the target folder,
**So that** I don't accidentally lose customizations.

### Acceptance Criteria
- [ ] AC-2.1: If `<folder>/rules/aws-aidlc-rules/` or `<folder>/aws-aidlc-rule-details/` already exists, the user is prompted "Rules already exist. Overwrite? (y/n)".
- [ ] AC-2.2: If the user answers "n", the tool exits gracefully with a message.
- [ ] AC-2.3: If the user answers "y", existing files are replaced.

---

## US-3: Gitignore — Rules Folder

**As** Alex,
**I want** to be asked whether to add the rules folder to `.gitignore`,
**So that** I can keep AI-DLC rules out of version control if desired.

### Acceptance Criteria
- [ ] AC-3.1: After installation, the user is prompted "Add `<folder>` to .gitignore? (y/n)".
- [ ] AC-3.2: If yes, the folder path is appended to `.gitignore` (file created if missing).
- [ ] AC-3.3: If the entry already exists in `.gitignore`, it is not duplicated.

---

## US-4: Gitignore — aidlc-docs Folder

**As** Alex,
**I want** to be asked whether to add `aidlc-docs/` to `.gitignore`,
**So that** AI-DLC generated artifacts don't pollute my repository.

### Acceptance Criteria
- [ ] AC-4.1: After the rules-folder gitignore prompt, the user is prompted "Add `aidlc-docs/` to .gitignore? (y/n)".
- [ ] AC-4.2: Same append/create/no-duplicate behavior as US-3.

---

## US-5: Network Error Handling

**As** Alex,
**I want** to see a clear, friendly error message if the download fails,
**So that** I understand what went wrong and what to do next.

### Acceptance Criteria
- [ ] AC-5.1: If the HTTP request fails (timeout, DNS, rate-limit, non-200 status), the spinner stops and a styled error message is shown.
- [ ] AC-5.2: The error message includes the reason (e.g. "Network error: could not reach github.com") and a suggestion (e.g. "Check your internet connection and try again.").
- [ ] AC-5.3: The tool exits with a non-zero exit code.

---

## US-6: Polished UI Experience

**As** Alex,
**I want** the CLI to feel professional with spinners, colors, and clear structure,
**So that** the experience is pleasant and trustworthy.

### Acceptance Criteria
- [ ] AC-6.1: Long-running operations (download, extraction) show an animated spinner.
- [ ] AC-6.2: Completed steps show a green ✓ checkmark.
- [ ] AC-6.3: Errors show a red ✗ with styled message.
- [ ] AC-6.4: Section headers and prompts use color/bold for readability.
- [ ] AC-6.5: The final output shows a tree view of installed files.

---

## US-7: Download Cache Across Projects

**As** Alex,
**I want** the tool to cache the downloaded release locally so that installing rules in a second project is instant,
**So that** I don't waste bandwidth or time re-downloading the same version.

### Acceptance Criteria
- [ ] AC-7.1: The zip is cached in a platform-appropriate cache directory, keyed by version tag.
- [ ] AC-7.2: On subsequent runs, if the cached version matches latest, the download is skipped and a message is shown ("Using cached release vX.Y.Z").
- [ ] AC-7.3: If a new version is available, the old cache is deleted and the new version is downloaded.
- [ ] AC-7.4: Cache cleanup is transparent — no manual action required from the user.

---

## US-8: Checksum Verification

**As** Alex,
**I want** the downloaded zip to be verified against a SHA-256 checksum,
**So that** I can trust the download hasn't been corrupted or tampered with.

### Acceptance Criteria
- [ ] AC-8.1: After download, SHA-256 is computed and verified against the GitHub release checksum asset (if available) or a stored checksum from first download.
- [ ] AC-8.2: If verification fails, the file is deleted and a clear error is shown.
- [ ] AC-8.3: The spinner shows "Verifying integrity..." during the check.

---

## US-9: Installed Rules Integrity Check

**As** Alex,
**I want** the tool to detect if installed rule files have been modified since installation,
**So that** I'm warned about potential tampering before overwriting.

### Acceptance Criteria
- [ ] AC-9.1: After installation, a `.aidlc-integrity.sha256` manifest is written with SHA-256 hashes of all installed `.md` files.
- [ ] AC-9.2: On subsequent runs (when overwrite is chosen), existing files are checked against the manifest.
- [ ] AC-9.3: If modifications are detected, a warning lists the changed files and asks for confirmation before proceeding.

---

## US-10: Secure Transport

**As** Alex,
**I want** all downloads to use HTTPS with certificate validation from the trusted GitHub repository only,
**So that** I'm protected against man-in-the-middle attacks and untrusted sources.

### Acceptance Criteria
- [ ] AC-10.1: All HTTP requests use HTTPS only.
- [ ] AC-10.2: TLS certificates are validated (no insecure mode).
- [ ] AC-10.3: Downloads only come from `https://github.com/awslabs/aidlc-workflows`.

---

## US-11: Commit Workflow Preference

**As** Alex,
**I want** to be asked about my preferred commit workflow during setup,
**So that** the AI-DLC rules enforce my team's commit conventions automatically.

### Acceptance Criteria
- [ ] AC-11.1: After installation, the user is prompted to choose a commit style: "Conventional Commits", "Free-form", or "None (I handle commits myself)".
- [ ] AC-11.2: If "Conventional Commits" is selected, a MANDATORY commit section is appended to `core-workflow.md` with conventional commit format rules and frequency guidance.
- [ ] AC-11.3: If "Free-form" is selected, a lighter commit reminder section is appended.
- [ ] AC-11.4: If "None" is selected, no commit rules are added.

---

## INVEST Validation

| Story | I | N | V | E | S | T |
|-------|---|---|---|---|---|---|
| US-1  | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| US-2  | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| US-3  | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| US-4  | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| US-5  | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| US-6  | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| US-7  | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| US-8  | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| US-9  | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| US-10 | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| US-11 | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |

All stories are Independent, Negotiable, Valuable, Estimable, Small, and Testable.
