//! ASCII art banner displayed on startup.
//!
//! Shows the tool identity with ASCII art and version.

use console::style;

/// Displays the startup banner with ASCII art and version.
pub fn print_banner() {
    let title = r#"
     _    ___      ____  _     ____
    / \  |_ _|    |  _ \| |   / ___|
   / _ \  | | ___ | | | | |  | |
  / ___ \ | ||___|| |_| | |__| |___
 /_/   \_\___|    |____/|_____\____|"#;
    println!("{}", style(title).cyan().bold());
    println!();
    println!(
        "  {} {}   {}",
        style("⚡").yellow(),
        style("Workflows Helper").bold(),
        style(format!("v{}", env!("CARGO_PKG_VERSION"))).dim()
    );
    println!(
        "  {}  {}",
        style("→").dim(),
        style("https://github.com/awslabs/aidlc-workflows").dim()
    );
}
