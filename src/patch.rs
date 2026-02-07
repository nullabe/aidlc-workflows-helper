//! Patches `core-workflow.md` after installation.
//!
//! Two kinds of patches are applied:
//! 1. **Path references** — The upstream `core-workflow.md` hardcodes paths like
//!    `.kiro/aws-aidlc-rule-details/`. These are replaced with the user's actual folder.
//! 2. **Commit workflow** — Optionally appends a commit convention section (Conventional
//!    Commits or free-form) based on the user's preference.

use crate::prompt::CommitWorkflow;
use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

/// Default paths that appear in core-workflow.md and need replacing.
const DEFAULT_PATHS: &[&str] = &[
    ".kiro/aws-aidlc-rule-details/",
    ".amazonq/aws-aidlc-rule-details/",
    ".aiassistant/aws-aidlc-rule-details/",
];

/// Patch core-workflow.md to replace default rule-details paths with the user's chosen path.
pub fn patch_rule_details_path(rules_folder: &str, details_parent: &str) -> Result<()> {
    let workflow_path = Path::new(rules_folder).join("rules/core-workflow.md");
    if !workflow_path.exists() {
        return Ok(());
    }

    let mut content =
        fs::read_to_string(&workflow_path).context("Failed to read core-workflow.md")?;

    let replacement = format!("{details_parent}/aws-aidlc-rule-details/");
    for default in DEFAULT_PATHS {
        content = content.replace(default, &replacement);
    }

    fs::write(&workflow_path, content)?;
    Ok(())
}

/// Append a commit workflow section to core-workflow.md based on user preference.
pub fn patch_commit_workflow(rules_folder: &str, workflow: &CommitWorkflow) -> Result<()> {
    let workflow_path = Path::new(rules_folder).join("rules/core-workflow.md");
    if !workflow_path.exists() {
        return Ok(());
    }

    let section = match workflow {
        CommitWorkflow::Conventional => CONVENTIONAL_SECTION,
        CommitWorkflow::FreeForm => FREEFORM_SECTION,
        CommitWorkflow::None => return Ok(()),
    };

    let mut content = fs::read_to_string(&workflow_path)?;
    content.push_str(section);
    fs::write(&workflow_path, content)?;
    Ok(())
}

const CONVENTIONAL_SECTION: &str = r#"

## MANDATORY: Commit Workflow
**CRITICAL**: Commit early and often using conventional commits. Do NOT accumulate large changes.

**Rules**:
1. After completing each logical unit of work, create a git commit.
2. Use conventional commit format: `feat:`, `fix:`, `docs:`, `ci:`, `chore:`, `refactor:`, `test:`.
3. Keep commits small and focused — one concern per commit.
4. Commit messages must be descriptive.
5. Never bundle unrelated changes in a single commit.
6. **Artifact commits**: Documentation artifacts (requirements, stories, plans, state updates) MUST also be committed. Use `docs:` prefix.
7. **Before moving to the next stage**, ensure all pending changes are committed.
"#;

const FREEFORM_SECTION: &str = r#"

## Commit Reminder
Remember to commit your changes regularly. Small, frequent commits are easier to review and revert.
Commit documentation artifacts (requirements, stories, plans) alongside code changes.
"#;

const RELATIVE_PATHS_SECTION: &str = r#"

## MANDATORY: Relative Paths Only
**CRITICAL**: All file and folder references in AI-DLC documents (aidlc-state.md, plans, requirements, stories, code summaries) MUST use paths relative to the workspace root.
- **NEVER** use absolute paths (e.g. `/Users/...`, `/home/...`, `C:\...`).
- The `Workspace Root` in `aidlc-state.md` MUST be `.` (dot), not an absolute path.
- This prevents leaking personal filesystem information into version control.
"#;

/// Append the relative-paths-only rule to core-workflow.md. Always applied.
pub fn patch_relative_paths_rule(rules_folder: &str) -> Result<()> {
    let workflow_path = Path::new(rules_folder).join("rules/core-workflow.md");
    if !workflow_path.exists() {
        return Ok(());
    }

    let mut content = fs::read_to_string(&workflow_path)?;
    content.push_str(RELATIVE_PATHS_SECTION);
    fs::write(&workflow_path, content)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn setup_workflow(dir: &Path, content: &str) -> String {
        let rules = dir.join("myfolder/rules");
        fs::create_dir_all(&rules).unwrap();
        fs::write(rules.join("core-workflow.md"), content).unwrap();
        dir.join("myfolder").to_string_lossy().to_string()
    }

    #[test]
    fn patches_kiro_path() {
        let dir = tempfile::tempdir().unwrap();
        let rules_folder = setup_workflow(
            dir.path(),
            "Load from `.kiro/aws-aidlc-rule-details/common/foo.md`",
        );
        patch_rule_details_path(&rules_folder, ".custom").unwrap();
        let result =
            fs::read_to_string(dir.path().join("myfolder/rules/core-workflow.md")).unwrap();
        assert!(result.contains(".custom/aws-aidlc-rule-details/common/foo.md"));
        assert!(!result.contains(".kiro/aws-aidlc-rule-details/"));
    }

    #[test]
    fn patches_amazonq_path() {
        let dir = tempfile::tempdir().unwrap();
        let rules_folder = setup_workflow(
            dir.path(),
            "Use `.amazonq/aws-aidlc-rule-details/` directory",
        );
        patch_rule_details_path(&rules_folder, ".myagent").unwrap();
        let result =
            fs::read_to_string(dir.path().join("myfolder/rules/core-workflow.md")).unwrap();
        assert!(result.contains(".myagent/aws-aidlc-rule-details/"));
    }

    #[test]
    fn appends_conventional_commit_section() {
        let dir = tempfile::tempdir().unwrap();
        let rules_folder = setup_workflow(dir.path(), "# Workflow\n");
        patch_commit_workflow(&rules_folder, &CommitWorkflow::Conventional).unwrap();
        let result =
            fs::read_to_string(dir.path().join("myfolder/rules/core-workflow.md")).unwrap();
        assert!(result.contains("MANDATORY: Commit Workflow"));
    }

    #[test]
    fn appends_freeform_section() {
        let dir = tempfile::tempdir().unwrap();
        let rules_folder = setup_workflow(dir.path(), "# Workflow\n");
        patch_commit_workflow(&rules_folder, &CommitWorkflow::FreeForm).unwrap();
        let result =
            fs::read_to_string(dir.path().join("myfolder/rules/core-workflow.md")).unwrap();
        assert!(result.contains("Commit Reminder"));
    }

    #[test]
    fn none_does_not_modify_file() {
        let dir = tempfile::tempdir().unwrap();
        let original = "# Workflow\n";
        let rules_folder = setup_workflow(dir.path(), original);
        patch_commit_workflow(&rules_folder, &CommitWorkflow::None).unwrap();
        let result =
            fs::read_to_string(dir.path().join("myfolder/rules/core-workflow.md")).unwrap();
        assert_eq!(result, original);
    }
}
