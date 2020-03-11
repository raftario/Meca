use crate::{config::Config, Result};
use anyhow::Context;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct VersionDownload {
    pub link: String,
    pub checksum: Option<String>,
}

#[derive(Deserialize)]
pub struct Version {
    pub name: String,
    #[cfg_attr(target_os = "windows", serde(rename = "windows"))]
    #[cfg_attr(target_os = "macos", serde(rename = "darwin"))]
    #[cfg_attr(target_os = "linux", serde(rename = "linux"))]
    pub download: VersionDownload,
}

#[derive(Deserialize)]
pub struct Metadata {
    pub terms: Option<String>,
    pub versions: Vec<Version>,
}

impl Metadata {
    pub fn fetch() -> Result<Self> {
        let config = Config::read()?;
        let contents = ureq::get(&config.ndk.metadata_url)
            .call()
            .into_string()
            .context("Can't fetch NDK versions metadata")?;
        serde_json::from_str(&contents).context("Invalid NDK versions metadata")
    }
}
