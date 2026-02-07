//! ASCII art banner displayed on startup.
//!
//! Shows the tool name, version, and a link to the upstream project.

use console::style;

/// Displays the startup banner with tool identity and version.
pub fn print_banner() {
    println!();
    println!(
        "  {}",
        style("┌─────────────────────────────────────────────┐").cyan()
    );
    println!(
        "  {}",
        style("│                                             │").cyan()
    );
    println!(
        "  {}  {}  {}",
        style("│").cyan(),
        style("⚡ AI-DLC Workflows Helper").bold().cyan(),
        style("              │").cyan()
    );
    println!(
        "  {}     {}  {}",
        style("│").cyan(),
        style(format!("v{}", env!("CARGO_PKG_VERSION"))).dim(),
        style("                          │").cyan()
    );
    println!(
        "  {}",
        style("│                                             │").cyan()
    );
    println!(
        "  {}",
        style("└─────────────────────────────────────────────┘").cyan()
    );
    println!();
    println!(
        "  {}  {}",
        style("→").dim(),
        style("Install AI-DLC workflow rules into any project").dim()
    );
    println!(
        "  {}  {}",
        style("→").dim(),
        style("https://github.com/awslabs/aidlc-workflows").dim()
    );
}
