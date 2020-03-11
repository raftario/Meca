use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct VersionDownload {
    link: String,
    checksum: Option<String>,
}

#[derive(Deserialize)]
pub struct Version {
    #[cfg_attr(target_os = "windows", serde(rename = "windows"))]
    #[cfg_attr(target_os = "macos", serde(rename = "darwin"))]
    #[cfg_attr(target_os = "linux", serde(rename = "linux"))]
    download: VersionDownload,
}

#[derive(Deserialize)]
pub struct Metadata {
    terms: Option<String>,
    #[serde(flatten)]
    versions: HashMap<String, Version>,
}
