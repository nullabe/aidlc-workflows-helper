use anyhow::{bail, Context, Result};
use sha2::{Digest, Sha256};
use std::fs;
use std::io::Write;
use std::path::Path;

/// Download a file from `url` to `dest`. Returns SHA-256 hex digest.
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

/// Verify a file's SHA-256 matches the expected hash.
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
