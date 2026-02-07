//! Styled terminal output helpers.
//!
//! Provides consistent formatting for success (✓), error (✗), info (ℹ), warning (⚠),
//! and section headers throughout the CLI. Uses the `console` crate for colors and styling.

use console::style;

/// Print a green checkmark with a completion message (e.g. "✓ Download complete").
pub fn step_done(msg: &str) {
    println!("  {} {}", style("✓").green().bold(), msg);
}

/// Print a red ✗ with an error message to stderr.
pub fn error(msg: &str) {
    eprintln!("  {} {}", style("✗").red().bold(), style(msg).red());
}

/// Print a cyan ℹ with an informational message.
pub fn info(msg: &str) {
    println!("  {} {}", style("ℹ").cyan(), msg);
}

/// Print a bold section header to visually separate workflow steps.
pub fn header(msg: &str) {
    println!("\n{}", style(msg).bold());
}

/// Print a yellow ⚠ with a warning message.
pub fn warn(msg: &str) {
    println!("  {} {}", style("⚠").yellow(), style(msg).yellow());
}
