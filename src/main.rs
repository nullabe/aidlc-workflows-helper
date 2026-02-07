//! aidlc-workflows-helper — CLI tool to install AI-DLC workflow rules into any project.
//!
//! This binary orchestrates the full interactive installation flow:
//! folder selection → download → extract → patch → commit workflow → gitignore → integrity manifest.

mod banner;
mod cache;
mod download;
mod extract;
mod github;
mod gitignore;
mod integrity;
mod patch;
mod prompt;
mod ui;

use anyhow::Result;
use console::style;
use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;

fn main() {
    if let Err(e) = run() {
        ui::error(&format!("{e:#}"));
        std::process::exit(1);
    }
}

/// Runs the full interactive installation flow. Returns an error if any step fails,
/// which `main()` catches and displays as a styled error message before exiting non-zero.
fn run() -> Result<()> {
    banner::print_banner();

    const TOTAL: u8 = 6;

    // ── Step 1: Folder selection ──
    ui::section(1, TOTAL, "📁 Where do you want to install AI-DLC rules?");
    let (rules_folder, details_parent) = prompt::select_folder()?;
    ui::step_done(&format!(
        "Rules     → {rules_folder}/rules/core-workflow.md"
    ));
    ui::step_done(&format!(
        "Details   → {details_parent}/aws-aidlc-rule-details/"
    ));

    // Check for existing rules + integrity
    if extract::rules_exist(&rules_folder, &details_parent) {
        let modified = integrity::verify_manifest(&details_parent)?;
        if !modified.is_empty() {
            ui::warn("These rule files have been modified since last installation:");
            for f in &modified {
                println!("      {}", style(format!("• {f}")).yellow());
            }
        }
        if !prompt::confirm_overwrite()? {
            ui::info("Skipped — no changes made.");
            return Ok(());
        }
    }

    // ── Step 2: Download ──
    ui::section(2, TOTAL, "🌐 Fetching latest AI-DLC rules");
    let spinner = make_spinner("Contacting GitHub...");
    let client = reqwest::blocking::Client::builder()
        .https_only(true)
        .build()?;
    let release = github::fetch_latest_release(&client)?;
    spinner.finish_and_clear();
    ui::step_done(&format!("Latest release: {}", style(&release.tag).bold()));

    let zip_path = cache::cached_zip_path(&release.tag)?;

    if cache::has_cached(&release.tag) {
        if let Some(expected) = cache::read_checksum(&release.tag)? {
            download::verify_checksum(&zip_path, &expected)?;
        }
        ui::info(&format!(
            "Using cached release {} — skipping download",
            style(&release.tag).bold()
        ));
    } else {
        let spinner = make_spinner("Downloading...");
        let checksum = download::download_to(&client, &release.zip_url, &zip_path)?;
        spinner.finish_and_clear();
        cache::store_checksum(&release.tag, &checksum)?;
        cache::cleanup_old_versions(&release.tag)?;
        ui::step_done("Downloaded and verified (SHA-256 ✓)");
    }

    // ── Step 3: Install ──
    ui::section(3, TOTAL, "📂 Installing rules");
    let spinner = make_spinner("Extracting...");
    let installed = extract::extract_and_install(&zip_path, &rules_folder, &details_parent)?;
    spinner.finish_and_clear();
    ui::step_done(&format!("{} files installed", installed.len()));

    // Patch core-workflow.md paths
    patch::patch_rule_details_path(&rules_folder, &details_parent)?;
    ui::step_done("Patched core-workflow.md path references");

    // Write integrity manifest
    integrity::write_manifest(&installed, &details_parent)?;
    ui::step_done("Integrity manifest written");

    // ── Step 4: Commit workflow ──
    ui::section(4, TOTAL, "📝 Commit workflow preference");
    let commit_pref = prompt::select_commit_workflow()?;
    patch::patch_commit_workflow(&rules_folder, &commit_pref)?;
    match commit_pref {
        prompt::CommitWorkflow::None => ui::info("No commit rules added"),
        _ => ui::step_done("Commit workflow patched into core-workflow.md"),
    }

    // ── Step 5: Gitignore ──
    ui::section(5, TOTAL, "🔒 Gitignore configuration");

    // FR-12: always add aidlc-docs/audit.md (no prompt)
    gitignore::add_to_gitignore("aidlc-docs/audit.md")?;
    ui::step_done("Auto-added aidlc-docs/audit.md to .gitignore");

    if prompt::confirm_gitignore_rules(&rules_folder)? {
        gitignore::add_to_gitignore(&format!("{rules_folder}/"))?;
        ui::step_done(&format!("Added {rules_folder}/ to .gitignore"));
    }
    if prompt::confirm_gitignore_aidlc_docs()? {
        gitignore::add_to_gitignore("aidlc-docs/")?;
        ui::step_done("Added aidlc-docs/ to .gitignore");
    }

    // ── Step 6: Done ──
    ui::section(6, TOTAL, "🎉 Summary");
    print_tree(&rules_folder, &details_parent);
    ui::success_box("Installation complete!");
    println!();
    ui::info("Start any AI-DLC workflow by telling your AI agent:");
    println!(
        "      {}",
        style("\"Using AI-DLC, I want to build ...\"").italic()
    );
    println!();

    Ok(())
}

/// Creates an animated spinner for long-running operations (download, extraction).
/// Call `.finish_and_clear()` when the operation completes.
fn make_spinner(msg: &str) -> ProgressBar {
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .template("  {spinner:.cyan} {msg}")
            .unwrap(),
    );
    pb.set_message(msg.to_string());
    pb.enable_steady_tick(Duration::from_millis(80));
    pb
}

/// Prints a visual tree of the installed file structure.
fn print_tree(rules_folder: &str, details_parent: &str) {
    println!("  {}", style("Installed:").dim());
    println!("  {}/", style(rules_folder).bold());
    println!("  └── rules/");
    println!("      └── core-workflow.md");
    println!("  {}/", style(details_parent).bold());
    println!("  └── aws-aidlc-rule-details/");
    println!("      ├── common/");
    println!("      ├── construction/");
    println!("      ├── inception/");
    println!("      └── operations/");
}
