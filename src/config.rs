use once_cell::sync::Lazy;
use std::{env, path::PathBuf};

pub static MECA_HOME: Lazy<PathBuf> = Lazy::new(|| match env::var_os("MECA_HOME") {
    Some(v) => PathBuf::from(v),
    None => dirs::home_dir().unwrap().join(".meca"),
});
