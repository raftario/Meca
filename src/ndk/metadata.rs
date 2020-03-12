use crate::{config::Config, with_progress, Result};
use anyhow::Context;
use serde::Deserialize;
use sha1::{Digest, Sha1};
use std::io::Read;

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

impl Version {
    pub fn download(&self) -> Result<Vec<u8>> {
        let response = ureq::get(&self.download.link).call();

        let mut result = match response.header("Content-Length") {
            Some(s) => match s.parse() {
                Ok(l) => Vec::with_capacity(l),
                Err(_) => Vec::new(),
            },
            None => Vec::new(),
        };
        response
            .into_reader()
            .read_to_end(&mut result)
            .context("Error downloading Android NDK")?;

        Ok(result)
    }
    pub fn download_with_progress(&self) -> Result<Vec<u8>> {
        Ok(with_progress!(self.download()?, "Downloading Android NDK"))
    }

    pub fn is_valid(&self, data: &[u8]) -> bool {
        let checksum = match &self.download.checksum {
            Some(s) => s,
            None => return true,
        };
        let data_checksum = format!("{:x}", Sha1::digest(data));
        &data_checksum == checksum
    }
    pub fn is_valid_with_progress(&self, data: &[u8]) -> bool {
        with_progress!(self.is_valid(data), "Validating checksum")
    }
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
            .set("Cache-Control", "no-cache, no-store")
            .call()
            .into_string()
            .context("Can't fetch NDK versions metadata")?;

        serde_json::from_str(&contents).context("Invalid NDK versions metadata")
    }
    pub fn fetch_with_progress() -> Result<Self> {
        Ok(with_progress!(Self::fetch()?, "Fetching metadata"))
    }
}
