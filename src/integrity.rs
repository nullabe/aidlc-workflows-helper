//! Integrity manifest for installed rule files.
//!
//! After installation, a `.aidlc-integrity.sha256` manifest is written containing
//! SHA-256 hashes of all installed `.md` files. On subsequent runs, the manifest is
//! checked to detect if any rule files have been modified since installation — this
//! warns users about potential tampering before they overwrite.

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
            let bytes =
                fs::read(path).with_context(|| format!("Failed to read {}", path.display()))?;
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn write_and_verify_unmodified() {
        let dir = tempfile::tempdir().unwrap();
        let parent = dir.path().to_string_lossy().to_string();

        let md_file = dir.path().join("test.md");
        fs::write(&md_file, "# Hello").unwrap();

        write_manifest(std::slice::from_ref(&md_file), &parent).unwrap();

        let manifest = dir.path().join(".aidlc-integrity.sha256");
        assert!(manifest.exists());

        let modified = verify_manifest(&parent).unwrap();
        assert!(modified.is_empty());
    }

    #[test]
    fn detects_modified_file() {
        let dir = tempfile::tempdir().unwrap();
        let parent = dir.path().to_string_lossy().to_string();

        let md_file = dir.path().join("test.md");
        fs::write(&md_file, "# Original").unwrap();

        write_manifest(std::slice::from_ref(&md_file), &parent).unwrap();

        // Tamper with the file
        fs::write(&md_file, "# Tampered").unwrap();

        let modified = verify_manifest(&parent).unwrap();
        assert_eq!(modified.len(), 1);
    }

    #[test]
    fn skips_non_md_files() {
        let dir = tempfile::tempdir().unwrap();
        let parent = dir.path().to_string_lossy().to_string();

        let txt_file = dir.path().join("readme.txt");
        fs::write(&txt_file, "hello").unwrap();

        write_manifest(&[txt_file], &parent).unwrap();

        let manifest = fs::read_to_string(dir.path().join(".aidlc-integrity.sha256")).unwrap();
        // Only a trailing newline — no entries for .txt files
        assert_eq!(manifest.trim(), "");
    }

    #[test]
    fn no_manifest_returns_empty() {
        let dir = tempfile::tempdir().unwrap();
        let parent = dir.path().to_string_lossy().to_string();
        let modified = verify_manifest(&parent).unwrap();
        assert!(modified.is_empty());
    }
}
