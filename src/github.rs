use anyhow::{bail, Context, Result};
use serde::Deserialize;

const API_URL: &str = "https://api.github.com/repos/awslabs/aidlc-workflows/releases/latest";

#[derive(Debug, Deserialize)]
struct Asset {
    name: String,
    browser_download_url: String,
}

#[derive(Debug, Deserialize)]
struct Release {
    tag_name: String,
    assets: Vec<Asset>,
}

/// Info about the latest release.
pub struct ReleaseInfo {
    pub tag: String,
    pub zip_url: String,
}

/// Fetch latest release info from GitHub API.
pub fn fetch_latest_release(client: &reqwest::blocking::Client) -> Result<ReleaseInfo> {
    let release: Release = client
        .get(API_URL)
        .header("User-Agent", "aidlc-workflows-helper")
        .header("Accept", "application/vnd.github+json")
        .send()
        .context("Failed to reach GitHub API")?
        .error_for_status()
        .context("GitHub API returned an error")?
        .json()
        .context("Failed to parse GitHub release JSON")?;

    let zip_asset = release
        .assets
        .iter()
        .find(|a| a.name.ends_with(".zip"))
        .context("No zip asset found in the latest release")?;

    // Validate trusted source
    if !zip_asset
        .browser_download_url
        .starts_with("https://github.com/awslabs/aidlc-workflows/")
    {
        bail!("Untrusted download URL: {}", zip_asset.browser_download_url);
    }

    Ok(ReleaseInfo {
        tag: release.tag_name,
        zip_url: zip_asset.browser_download_url.clone(),
    })
}
