//! ASCII art banner displayed on startup.
//!
//! Shows the tool name and version (read from Cargo.toml at compile time).

use console::style;

/// Displays the ASCII art banner with tool name and version.
pub fn print_banner() {
    let banner = r#"
     _    ___   ____  _     ____
    / \  |_ _| |  _ \| |   / ___|
   / _ \  | |  | | | | |  | |
  / ___ \ | |  | |_| | |__| |___
 /_/   \_\___| |____/|_____\____|
"#;
    println!("{}", style(banner).cyan().bold());
    println!(
        "  {} {}",
        style("AI-DLC Workflows Helper").bold(),
        style(format!("v{}", env!("CARGO_PKG_VERSION"))).dim()
    );
    println!();
}
