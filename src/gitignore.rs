//! `.gitignore` file manipulation.
//!
//! Appends entries to `.gitignore` with deduplication (won't add an entry that already
//! exists, even if the trailing slash differs). Creates the file if it doesn't exist.

use anyhow::Result;
use std::fs;
use std::path::Path;

/// Add an entry to .gitignore if not already present. Creates the file if missing.
pub fn add_to_gitignore(entry: &str) -> Result<()> {
    add_entry(Path::new(".gitignore"), entry)
}

/// Testable core: add entry to a gitignore file at the given path.
fn add_entry(path: &Path, entry: &str) -> Result<()> {
    let mut content = if path.exists() {
        fs::read_to_string(path)?
    } else {
        String::new()
    };

    let normalized = entry.trim_end_matches('/');
    let already_present = content.lines().any(|line| {
        let l = line.trim().trim_end_matches('/');
        l == normalized
    });

    if !already_present {
        if !content.is_empty() && !content.ends_with('\n') {
            content.push('\n');
        }
        content.push_str(entry);
        content.push('\n');
        fs::write(path, content)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn adds_entry_to_new_file() {
        let dir = tempfile::tempdir().unwrap();
        let gi = dir.path().join(".gitignore");
        add_entry(&gi, "target/").unwrap();
        assert_eq!(fs::read_to_string(&gi).unwrap(), "target/\n");
    }

    #[test]
    fn appends_entry_to_existing_file() {
        let dir = tempfile::tempdir().unwrap();
        let gi = dir.path().join(".gitignore");
        fs::write(&gi, "node_modules\n").unwrap();
        add_entry(&gi, "target/").unwrap();
        assert_eq!(fs::read_to_string(&gi).unwrap(), "node_modules\ntarget/\n");
    }

    #[test]
    fn does_not_duplicate_entry() {
        let dir = tempfile::tempdir().unwrap();
        let gi = dir.path().join(".gitignore");
        fs::write(&gi, "target/\n").unwrap();
        add_entry(&gi, "target/").unwrap();
        assert_eq!(fs::read_to_string(&gi).unwrap(), "target/\n");
    }

    #[test]
    fn normalizes_trailing_slash() {
        let dir = tempfile::tempdir().unwrap();
        let gi = dir.path().join(".gitignore");
        fs::write(&gi, "target\n").unwrap();
        add_entry(&gi, "target/").unwrap();
        // "target" already present, so "target/" should not be added
        assert_eq!(fs::read_to_string(&gi).unwrap(), "target\n");
    }
}
