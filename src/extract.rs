use anyhow::{Context, Result};
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

/// Extract the zip and install rules into the target folders.
/// `rules_folder` is where aws-aidlc-rules/ contents go (e.g. ".kiro/steering").
/// `details_parent` is where aws-aidlc-rule-details/ goes (e.g. ".kiro").
/// Returns list of installed file paths (relative to project root).
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

        // Files inside the zip are under aidlc-rules/aws-aidlc-rules/ and aidlc-rules/aws-aidlc-rule-details/
        if let Some(rel) = entry_path.strip_prefix("aidlc-rules/aws-aidlc-rules/") {
            if rel.is_empty() || entry.is_dir() {
                continue;
            }
            let dest = PathBuf::from(rules_folder).join("aws-aidlc-rules").join(rel);
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

fn write_entry(entry: &mut zip::read::ZipFile, dest: &Path) -> Result<()> {
    if let Some(parent) = dest.parent() {
        fs::create_dir_all(parent)?;
    }
    let mut out = fs::File::create(dest)?;
    io::copy(entry, &mut out)?;
    Ok(())
}

/// Check if rules already exist in the target folder.
pub fn rules_exist(rules_folder: &str, details_parent: &str) -> bool {
    PathBuf::from(rules_folder).join("aws-aidlc-rules").exists()
        || PathBuf::from(details_parent)
            .join("aws-aidlc-rule-details")
            .exists()
}
