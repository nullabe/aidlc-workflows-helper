use console::style;

/// Print a green checkmark with message.
pub fn step_done(msg: &str) {
    println!("  {} {}", style("✓").green().bold(), msg);
}

/// Print a styled error message (red ✗).
pub fn error(msg: &str) {
    eprintln!("  {} {}", style("✗").red().bold(), style(msg).red());
}

/// Print an info message (cyan).
pub fn info(msg: &str) {
    println!("  {} {}", style("ℹ").cyan(), msg);
}

/// Print a section header (bold).
pub fn header(msg: &str) {
    println!("\n{}", style(msg).bold());
}

/// Print a warning message (yellow).
pub fn warn(msg: &str) {
    println!("  {} {}", style("⚠").yellow(), style(msg).yellow());
}
