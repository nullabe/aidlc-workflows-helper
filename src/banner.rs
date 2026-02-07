//! ASCII art banner displayed on startup.
//!
//! Shows the tool identity with an AWS Labs branded ASCII art logo.

use console::style;

/// Displays the startup banner with ASCII art and version.
pub fn print_banner() {
    let logo = r#"
       ___        ______    _          _
      / \ \      / / ___|  | |    __ _| |__  ___
     / _ \ \ /\ / /\___ \  | |   / _` | '_ \/ __|
    / ___ \ V  V /  ___) | | |__| (_| | |_) \__ \
   /_/   \_\_/\_/  |____/  |_____\__,_|_.__/|___/
"#;
    println!("{}", style(logo).yellow());

    let title = r#"     _    ___      ____  _     ____
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
