use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::{env, path::PathBuf};

pub static MECA_HOME: Lazy<PathBuf> = Lazy::new(|| match env::var_os("MECA_HOME") {
    Some(v) => PathBuf::from(v),
    None => dirs::home_dir().unwrap().join(".meca"),
});

#[derive(Deserialize, Serialize)]
#[serde(default)]
pub struct Config {
    resources_url: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            resources_url:
                "https://raw.githubusercontent.com/raftario/Meca/master/resources/metadata.json"
                    .to_owned(),
        }
    }
}
