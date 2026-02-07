//! Styled terminal output helpers.
//!
//! Provides consistent formatting for success (✓), error (✗), info (ℹ), warning (⚠),
//! and section headers throughout the CLI. Uses the `console` crate for colors and styling.

use console::style;

/// Print a green checkmark with a completion message.
pub fn step_done(msg: &str) {
    println!("  {} {}", style("✓").green().bold(), msg);
}

/// Print a red ✗ with an error message to stderr.
pub fn error(msg: &str) {
    eprintln!("\n  {} {}", style("✗").red().bold(), style(msg).red());
}

/// Print a cyan ℹ with an informational message.
pub fn info(msg: &str) {
    println!("  {} {}", style("ℹ").cyan(), msg);
}

/// Print a numbered section header with a visual separator.
pub fn section(step: u8, total: u8, msg: &str) {
    println!();
    println!(
        "  {} {}",
        style(format!("[{step}/{total}]")).dim().bold(),
        style(msg).bold()
    );
    println!("  {}", style("─".repeat(45)).dim());
}

/// Print a yellow ⚠ with a warning message.
pub fn warn(msg: &str) {
    println!("  {} {}", style("⚠").yellow(), style(msg).yellow());
}

/// Print a final success box.
pub fn success_box(msg: &str) {
    println!();
    println!(
        "  {}",
        style("╔═══════════════════════════════════════════╗").green()
    );
    println!(
        "  {}  {}  {}",
        style("║").green(),
        style(format!("✓ {msg}")).green().bold(),
        style("║").green()
    );
    println!(
        "  {}",
        style("╚═══════════════════════════════════════════╝").green()
    );
}
