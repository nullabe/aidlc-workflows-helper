use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;

const CACHE_DIR_NAME: &str = "aidlc-workflows-helper";

/// Get the platform-appropriate cache directory.
fn cache_root() -> Result<PathBuf> {
    let base = dirs::cache_dir().context("Could not determine cache directory")?;
    Ok(base.join(CACHE_DIR_NAME))
}

/// Get the path where a zip for a given version tag would be cached.
pub fn cached_zip_path(tag: &str) -> Result<PathBuf> {
    Ok(cache_root()?.join(tag).join("aidlc-rules.zip"))
}

/// Get the path where the checksum for a cached zip is stored.
pub fn cached_checksum_path(tag: &str) -> Result<PathBuf> {
    Ok(cache_root()?.join(tag).join("sha256"))
}

/// Check if a cached zip exists for the given tag.
pub fn has_cached(tag: &str) -> bool {
    cached_zip_path(tag).map(|p| p.exists()).unwrap_or(false)
}

/// Store checksum alongside the cached zip.
pub fn store_checksum(tag: &str, checksum: &str) -> Result<()> {
    let path = cached_checksum_path(tag)?;
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(&path, checksum)?;
    Ok(())
}

/// Read stored checksum for a cached version.
pub fn read_checksum(tag: &str) -> Result<Option<String>> {
    let path = cached_checksum_path(tag)?;
    if path.exists() {
        Ok(Some(fs::read_to_string(&path)?.trim().to_string()))
    } else {
        Ok(None)
    }
}

/// Delete all cached versions except the given tag.
pub fn cleanup_old_versions(keep_tag: &str) -> Result<()> {
    let root = cache_root()?;
    if !root.exists() {
        return Ok(());
    }
    for entry in fs::read_dir(&root)? {
        let entry = entry?;
        if entry.file_type()?.is_dir() {
            let name = entry.file_name();
            if name.to_string_lossy() != keep_tag {
                fs::remove_dir_all(entry.path()).ok();
            }
        }
    }
    Ok(())
}
