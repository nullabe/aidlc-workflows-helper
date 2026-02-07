use anyhow::{Context, Result};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

const MANIFEST_NAME: &str = ".aidlc-integrity.sha256";

/// Compute SHA-256 hashes of all installed .md files and write a manifest.
pub fn write_manifest(installed_files: &[PathBuf], details_parent: &str) -> Result<()> {
    let manifest_path = Path::new(details_parent).join(MANIFEST_NAME);
    let mut lines = Vec::new();

    for path in installed_files {
        if path.extension().is_some_and(|e| e == "md") {
            let bytes = fs::read(path)
                .with_context(|| format!("Failed to read {}", path.display()))?;
            let hash = format!("{:x}", Sha256::digest(&bytes));
            lines.push(format!("{}  {}", hash, path.display()));
        }
    }

    lines.sort();
    let mut content = lines.join("\n");
    content.push('\n');
    fs::write(&manifest_path, content)?;
    Ok(())
}

/// Verify existing files against the manifest. Returns list of modified file paths.
pub fn verify_manifest(details_parent: &str) -> Result<Vec<String>> {
    let manifest_path = Path::new(details_parent).join(MANIFEST_NAME);
    if !manifest_path.exists() {
        return Ok(Vec::new());
    }

    let content = fs::read_to_string(&manifest_path)?;
    let expected: BTreeMap<String, String> = content
        .lines()
        .filter_map(|line| {
            let (hash, path) = line.split_once("  ")?;
            Some((path.to_string(), hash.to_string()))
        })
        .collect();

    let mut modified = Vec::new();
    for (path, expected_hash) in &expected {
        let file_path = Path::new(path);
        if file_path.exists() {
            let bytes = fs::read(file_path)?;
            let actual = format!("{:x}", Sha256::digest(&bytes));
            if actual != *expected_hash {
                modified.push(path.clone());
            }
        }
    }

    Ok(modified)
}
