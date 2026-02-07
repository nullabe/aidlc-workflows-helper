//! HTTPS file download with SHA-256 checksum verification.
//!
//! Downloads are performed over HTTPS only (enforced by the `reqwest` client configured
//! with `https_only(true)` in `main.rs`). The checksum is computed during download and
//! returned to the caller for storage in the cache.

use anyhow::{Context, Result, bail};
use sha2::{Digest, Sha256};
use std::fs;
use std::io::Write;
use std::path::Path;

/// Download a file from `url` to `dest`, computing its SHA-256 hash on the fly.
///
/// Creates parent directories if they don't exist. Returns the hex-encoded SHA-256
/// digest of the downloaded content.
pub fn download_to(client: &reqwest::blocking::Client, url: &str, dest: &Path) -> Result<String> {
    if let Some(parent) = dest.parent() {
        fs::create_dir_all(parent)?;
    }

    let resp = client
        .get(url)
        .header("User-Agent", "aidlc-workflows-helper")
        .send()
        .context("Download failed")?
        .error_for_status()
        .context("Download returned an error status")?;

    let bytes = resp.bytes().context("Failed to read download body")?;

    let mut hasher = Sha256::new();
    hasher.update(&bytes);
    let hash = format!("{:x}", hasher.finalize());

    let mut file = fs::File::create(dest)?;
    file.write_all(&bytes)?;

    Ok(hash)
}

/// Verify that a file's SHA-256 hash matches `expected`.
///
/// If the hash doesn't match, the file is deleted and an error is returned.
/// This prevents using corrupted or tampered downloads.
pub fn verify_checksum(path: &Path, expected: &str) -> Result<()> {
    let bytes = fs::read(path).context("Failed to read file for checksum")?;
    let mut hasher = Sha256::new();
    hasher.update(&bytes);
    let actual = format!("{:x}", hasher.finalize());

    if actual != expected {
        fs::remove_file(path).ok();
        bail!(
            "Checksum mismatch!\n  Expected: {expected}\n  Got:      {actual}\n\nCorrupted file has been deleted."
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn verify_checksum_passes_for_correct_hash() {
        let dir = tempfile::tempdir().unwrap();
        let file = dir.path().join("test.bin");
        fs::write(&file, b"hello world").unwrap();

        let expected = "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9";
        verify_checksum(&file, expected).unwrap();
    }

    #[test]
    fn verify_checksum_fails_and_deletes_file() {
        let dir = tempfile::tempdir().unwrap();
        let file = dir.path().join("test.bin");
        fs::write(&file, b"hello world").unwrap();

        let result = verify_checksum(
            &file,
            "0000000000000000000000000000000000000000000000000000000000000000",
        );
        assert!(result.is_err());
        assert!(!file.exists(), "corrupted file should be deleted");
    }
}
