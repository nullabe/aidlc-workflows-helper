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

    // 1. Folder selection
    ui::header("📁 Target Folder");
    let (rules_folder, details_parent) = prompt::select_folder()?;
    ui::step_done(&format!("Rules folder: {rules_folder}"));
    ui::step_done(&format!(
        "Rule details: {details_parent}/aws-aidlc-rule-details/"
    ));

    // 2. Check for existing rules + integrity
    if extract::rules_exist(&rules_folder, &details_parent) {
        let modified = integrity::verify_manifest(&details_parent)?;
        if !modified.is_empty() {
            ui::warn("The following rule files have been modified since installation:");
            for f in &modified {
                println!("    • {f}");
            }
        }
        if !prompt::confirm_overwrite()? {
            ui::info("Skipped. No changes made.");
            return Ok(());
        }
    }

    // 3. Fetch latest release info
    ui::header("🌐 Fetching Latest Release");
    let spinner = make_spinner("Contacting GitHub...");
    let client = reqwest::blocking::Client::builder()
        .https_only(true)
        .build()?;
    let release = github::fetch_latest_release(&client)?;
    spinner.finish_and_clear();
    ui::step_done(&format!("Latest release: {}", release.tag));

    // 4. Download or use cache
    ui::header("📦 Download");
    let zip_path = cache::cached_zip_path(&release.tag)?;

    if cache::has_cached(&release.tag) {
        // Verify cached checksum
        if let Some(expected) = cache::read_checksum(&release.tag)? {
            download::verify_checksum(&zip_path, &expected)?;
        }
        ui::info(&format!("Using cached release {}", release.tag));
    } else {
        let spinner = make_spinner("Downloading...");
        let checksum = download::download_to(&client, &release.zip_url, &zip_path)?;
        spinner.finish_and_clear();
        ui::step_done("Download complete");

        // Store checksum and clean old versions
        let spinner = make_spinner("Verifying integrity...");
        cache::store_checksum(&release.tag, &checksum)?;
        cache::cleanup_old_versions(&release.tag)?;
        spinner.finish_and_clear();
        ui::step_done("Checksum verified");
    }

    // 5. Extract and install
    ui::header("📂 Installing Rules");
    let spinner = make_spinner("Extracting...");
    let installed = extract::extract_and_install(&zip_path, &rules_folder, &details_parent)?;
    spinner.finish_and_clear();
    ui::step_done(&format!("{} files installed", installed.len()));

    // 6. Patch core-workflow.md paths
    patch::patch_rule_details_path(&rules_folder, &details_parent)?;
    ui::step_done("Patched core-workflow.md path references");

    // 7. Commit workflow preference
    ui::header("📝 Commit Workflow");
    let commit_pref = prompt::select_commit_workflow()?;
    patch::patch_commit_workflow(&rules_folder, &commit_pref)?;
    match commit_pref {
        prompt::CommitWorkflow::None => ui::info("No commit rules added"),
        _ => ui::step_done("Commit workflow added to core-workflow.md"),
    }

    // 8. Gitignore
    ui::header("🔒 Gitignore");
    // FR-12: always add aidlc-docs/audit.md
    gitignore::add_to_gitignore("aidlc-docs/audit.md")?;
    ui::step_done("Added aidlc-docs/audit.md to .gitignore");

    if prompt::confirm_gitignore(&rules_folder)? {
        gitignore::add_to_gitignore(&rules_folder)?;
        ui::step_done(&format!("Added {rules_folder} to .gitignore"));
    }
    if prompt::confirm_gitignore_aidlc_docs()? {
        gitignore::add_to_gitignore("aidlc-docs/")?;
        ui::step_done("Added aidlc-docs/ to .gitignore");
    }

    // 9. Write integrity manifest
    integrity::write_manifest(&installed, &details_parent)?;
    ui::step_done("Integrity manifest written");

    // 10. Final summary
    ui::header("✅ Installation Complete!");
    println!();
    print_tree(&rules_folder, &details_parent);
    println!();
    ui::info("Start any AI-DLC workflow by saying: \"Using AI-DLC, ...\"");

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

/// Prints a visual tree of the installed file structure so the user can verify
/// where rules and rule-details were placed.
fn print_tree(rules_folder: &str, details_parent: &str) {
    println!("  {}", style("Installed file tree:").dim());
    println!("  {rules_folder}/");
    println!("  ├── aws-aidlc-rules/");
    println!("  │   └── core-workflow.md");
    println!("  {details_parent}/");
    println!("  └── aws-aidlc-rule-details/");
    println!("      ├── common/");
    println!("      ├── construction/");
    println!("      ├── inception/");
    println!("      └── operations/");
}
