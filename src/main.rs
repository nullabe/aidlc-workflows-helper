mod banner;
mod cache;
mod download;
mod extract;
mod github;
mod gitignore;
mod integrity;
mod patch;
mod prompt;
mod ui;

fn main() {
    if let Err(e) = run() {
        ui::error(&format!("{e:#}"));
        std::process::exit(1);
    }
}

fn run() -> anyhow::Result<()> {
    // Will be wired in Step 10
    banner::print_banner();
    Ok(())
}
