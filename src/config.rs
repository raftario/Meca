use crate::Result;
use anyhow::Context;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::{
    env, fs,
    path::{Path, PathBuf},
};

pub static MECA_HOME: Lazy<PathBuf> = Lazy::new(|| match env::var_os("MECA_HOME") {
    Some(v) => PathBuf::from(v),
    None => dirs::home_dir().unwrap().join(".meca"),
});
pub static MECA_CONFIG_PATH: Lazy<PathBuf> = Lazy::new(|| MECA_HOME.join("Config.toml"));

#[derive(Deserialize, Serialize)]
pub struct NdkInstall {
    pub name: String,
    pub path: PathBuf,
}

#[derive(Deserialize, Serialize)]
#[serde(default)]
pub struct NdkConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub installed: Vec<NdkInstall>,
    pub install_dir: PathBuf,
    pub metadata_url: String,
}

impl NdkConfig {
    pub fn install_dir(&self) -> PathBuf {
        if self.install_dir.is_absolute() {
            self.install_dir.clone()
        } else {
            MECA_HOME.join(&self.install_dir)
        }
    }
}

impl Default for NdkConfig {
    fn default() -> Self {
        Self {
            selected: None,
            installed: vec![],
            install_dir: Path::new("ndk").join("installs"),
            metadata_url:
                "https://raw.githubusercontent.com/raftario/Meca/master/resources/metadata.json"
                    .to_owned(),
        }
    }
}

#[derive(Deserialize, Serialize, Default)]
#[serde(default)]
pub struct Config {
    pub ndk: NdkConfig,
}

impl Config {
    pub fn read() -> Result<Self> {
        if !MECA_HOME.exists() {
            fs::create_dir_all(&*MECA_HOME).context("Can't create `MECA_HOME`")?;
        }
        if MECA_CONFIG_PATH.exists() {
            let contents =
                fs::read_to_string(&*MECA_CONFIG_PATH).context("Can't read global Meca config")?;
            toml::from_str(&contents).context("Invalid global Meca config")
        } else {
            let config = Config::default();
            config.write()?;
            Ok(config)
        }
    }
    pub fn write(&self) -> Result<()> {
        let contents = toml::to_string(&self)?;
        fs::write(&*MECA_CONFIG_PATH, &contents).context("Can't write global Meca config")
    }
}
