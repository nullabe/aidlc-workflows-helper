//! Interactive CLI prompts for user input.
//!
//! Handles all user-facing prompts: folder selection (with presets for Kiro, Amazon Q,
//! Cursor), overwrite confirmation, gitignore options, and commit workflow preference.
//! Uses `dialoguer` for styled interactive selection and confirmation.

use anyhow::Result;
use dialoguer::{Confirm, Input, Select};

/// A folder preset shown in the interactive selection menu.
struct Preset {
    label: &'static str,
    path: &'static str,
    desc: &'static str,
}

const PRESETS: &[Preset] = &[
    Preset {
        label: ".kiro/steering",
        path: ".kiro/steering",
        desc: "Kiro IDE / Kiro CLI steering files",
    },
    Preset {
        label: ".amazonq/rules",
        path: ".amazonq/rules",
        desc: "Amazon Q Developer IDE plugin",
    },
    Preset {
        label: ".cursor/rules",
        path: ".cursor/rules",
        desc: "Cursor AI editor",
    },
];

/// Prompt user to select a target folder for rules installation.
/// Returns (rules_folder, rule_details_parent) — e.g. (".kiro/steering", ".kiro").
pub fn select_folder() -> Result<(String, String)> {
    let mut items: Vec<String> = PRESETS
        .iter()
        .map(|p| format!("{} — {}", p.label, p.desc))
        .collect();
    items.push("Custom path".to_string());

    let selection = Select::new()
        .with_prompt("Where should AI-DLC rules be installed?")
        .items(&items)
        .default(0)
        .interact()?;

    let rules_folder = if selection < PRESETS.len() {
        PRESETS[selection].path.to_string()
    } else {
        Input::new()
            .with_prompt("Enter custom folder path (relative to project root)")
            .interact_text()?
    };

    // rule-details go one level up from the rules folder
    let parent = rules_folder
        .rsplit_once('/')
        .map(|(p, _)| p.to_string())
        .unwrap_or_else(|| rules_folder.clone());

    Ok((rules_folder, parent))
}

/// Ask whether to overwrite existing rules.
pub fn confirm_overwrite() -> Result<bool> {
    Ok(Confirm::new()
        .with_prompt("Rules already exist. Overwrite?")
        .default(false)
        .interact()?)
}

/// Ask whether to add a path to .gitignore.
pub fn confirm_gitignore(path: &str) -> Result<bool> {
    Ok(Confirm::new()
        .with_prompt(format!("Add `{path}` to .gitignore?"))
        .default(true)
        .interact()?)
}

/// Ask whether to add aidlc-docs/ to .gitignore.
pub fn confirm_gitignore_aidlc_docs() -> Result<bool> {
    Ok(Confirm::new()
        .with_prompt("Add `aidlc-docs/` to .gitignore?")
        .default(true)
        .interact()?)
}

/// User's preferred commit workflow, selected during setup.
/// Determines what (if anything) gets appended to `core-workflow.md`.
pub enum CommitWorkflow {
    Conventional,
    FreeForm,
    None,
}

/// Ask user about their preferred commit workflow.
pub fn select_commit_workflow() -> Result<CommitWorkflow> {
    let items = &[
        "Conventional Commits — feat:, fix:, docs:, etc.",
        "Free-form — just a reminder to commit regularly",
        "None — I handle commits myself",
    ];

    let selection = Select::new()
        .with_prompt("Preferred commit workflow for AI-DLC?")
        .items(items)
        .default(0)
        .interact()?;

    Ok(match selection {
        0 => CommitWorkflow::Conventional,
        1 => CommitWorkflow::FreeForm,
        _ => CommitWorkflow::None,
    })
}
