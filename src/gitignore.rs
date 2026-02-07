use anyhow::Result;
use std::fs;
use std::path::Path;

/// Add an entry to .gitignore if not already present. Creates the file if missing.
pub fn add_to_gitignore(entry: &str) -> Result<()> {
    let path = Path::new(".gitignore");
    let mut content = if path.exists() {
        fs::read_to_string(path)?
    } else {
        String::new()
    };

    // Check if entry already exists (exact line match)
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
