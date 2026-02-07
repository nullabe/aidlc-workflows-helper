//! Zip extraction and rule file installation.
//!
//! Extracts the AI-DLC release zip and copies files into the user's chosen folder structure.
//! The zip contains `aidlc-rules/aws-aidlc-rules/` (the core workflow) and
//! `aidlc-rules/aws-aidlc-rule-details/` (supporting documents). These are mapped to:
//!
//! - `<rules_folder>/rules/` — e.g. `.kiro/steering/rules/core-workflow.md`
//! - `<details_parent>/aws-aidlc-rule-details/` — e.g. `.kiro/aws-aidlc-rule-details/`

use anyhow::{Context, Result};
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

/// Extract the zip and install rules into the target folders.
///
/// `aws-aidlc-rules/` from the zip is installed as `<rules_folder>/rules/` (renamed).
/// `aws-aidlc-rule-details/` is installed as `<details_parent>/aws-aidlc-rule-details/`.
///
/// Returns list of installed file paths relative to the project root.
pub fn extract_and_install(
    zip_path: &Path,
    rules_folder: &str,
    details_parent: &str,
) -> Result<Vec<PathBuf>> {
    let file = fs::File::open(zip_path).context("Failed to open zip file")?;
    let mut archive = zip::ZipArchive::new(file).context("Failed to read zip archive")?;

    let mut installed = Vec::new();

    for i in 0..archive.len() {
        let mut entry = archive.by_index(i)?;
        let entry_path = entry.name().to_string();

        // aws-aidlc-rules/ from zip → rules/ in target
        if let Some(rel) = entry_path.strip_prefix("aidlc-rules/aws-aidlc-rules/") {
            if rel.is_empty() || entry.is_dir() {
                continue;
            }
            let dest = PathBuf::from(rules_folder).join("rules").join(rel);
            write_entry(&mut entry, &dest)?;
            installed.push(dest);
        } else if let Some(rel) = entry_path.strip_prefix("aidlc-rules/aws-aidlc-rule-details/") {
            if rel.is_empty() || entry.is_dir() {
                continue;
            }
            let dest = PathBuf::from(details_parent)
                .join("aws-aidlc-rule-details")
                .join(rel);
            write_entry(&mut entry, &dest)?;
            installed.push(dest);
        }
    }

    Ok(installed)
}

fn write_entry(entry: &mut zip::read::ZipFile<impl std::io::Read>, dest: &Path) -> Result<()> {
    if let Some(parent) = dest.parent() {
        fs::create_dir_all(parent)?;
    }
    let mut out = fs::File::create(dest)?;
    io::copy(entry, &mut out)?;
    Ok(())
}

/// Check if rules already exist in the target folder.
pub fn rules_exist(rules_folder: &str, details_parent: &str) -> bool {
    PathBuf::from(rules_folder).join("rules").exists()
        || PathBuf::from(details_parent)
            .join("aws-aidlc-rule-details")
            .exists()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    /// Create a test zip with the expected aidlc-rules structure.
    fn create_test_zip(dir: &Path) -> PathBuf {
        let zip_path = dir.join("test.zip");
        let file = fs::File::create(&zip_path).unwrap();
        let mut zip = zip::ZipWriter::new(file);
        let options = zip::write::SimpleFileOptions::default();

        zip.start_file("aidlc-rules/aws-aidlc-rules/core-workflow.md", options)
            .unwrap();
        zip.write_all(b"# Core Workflow").unwrap();

        zip.start_file(
            "aidlc-rules/aws-aidlc-rule-details/common/process-overview.md",
            options,
        )
        .unwrap();
        zip.write_all(b"# Process Overview").unwrap();

        zip.finish().unwrap();
        zip_path
    }

    #[test]
    fn extracts_rules_to_rules_dir_and_details() {
        let dir = tempfile::tempdir().unwrap();
        let zip_path = create_test_zip(dir.path());

        let rules_folder = dir.path().join("steering");
        let details_parent = dir.path().join("kiro");

        let installed = extract_and_install(
            &zip_path,
            &rules_folder.to_string_lossy(),
            &details_parent.to_string_lossy(),
        )
        .unwrap();

        assert_eq!(installed.len(), 2);

        // aws-aidlc-rules/ from zip becomes rules/ in target
        let workflow = rules_folder.join("rules/core-workflow.md");
        assert!(workflow.exists());
        assert_eq!(fs::read_to_string(&workflow).unwrap(), "# Core Workflow");

        let overview = details_parent.join("aws-aidlc-rule-details/common/process-overview.md");
        assert!(overview.exists());
        assert_eq!(fs::read_to_string(&overview).unwrap(), "# Process Overview");
    }

    #[test]
    fn rules_exist_detects_existing_folders() {
        let dir = tempfile::tempdir().unwrap();
        let rules = dir.path().join("steering/rules");
        fs::create_dir_all(&rules).unwrap();

        assert!(rules_exist(
            &dir.path().join("steering").to_string_lossy(),
            &dir.path().join("other").to_string_lossy(),
        ));
    }

    #[test]
    fn rules_exist_returns_false_when_empty() {
        let dir = tempfile::tempdir().unwrap();
        assert!(!rules_exist(
            &dir.path().join("steering").to_string_lossy(),
            &dir.path().join("other").to_string_lossy(),
        ));
    }
}
