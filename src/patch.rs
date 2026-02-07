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
    let workflow_path = Path::new(rules_folder).join("aws-aidlc-rules/core-workflow.md");
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
    let workflow_path = Path::new(rules_folder).join("aws-aidlc-rules/core-workflow.md");
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
