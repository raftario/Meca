use crate::{
    config::Config,
    globals::{STDERR, STDOUT},
    ndk::metadata::Metadata,
    Result,
};
use dialoguer::{Confirmation, Select};
use std::{io::Cursor, path::PathBuf};
use structopt::StructOpt;
use zip::ZipArchive;

#[derive(StructOpt, Debug)]
pub struct Command {
    #[structopt(short = "y", long = "agree")]
    /// Skips prompts and automatically accepts the terms & conditions
    agree: bool,

    #[structopt(
        short = "V",
        long = "version",
        name = "VERSION",
        required_if("agree", "true")
    )]
    /// Specifies the version to install
    version: Option<String>,

    #[structopt(
        short = "p",
        long = "path",
        name = "PATH",
        required_if("agree", "true")
    )]
    /// Specifies the path of the installation directory
    path: Option<PathBuf>,
}

impl Command {
    pub fn run(self) -> Result<()> {
        let config = Config::read()?;

        if let Some(v) = &self.version {
            if config.ndk.installed.iter().any(|i| &i.name == v) {
                STDERR.write_line(&format!("Android NDK {} is already installed.", v))?;
                return Ok(());
            }
        }

        let metadata = Metadata::fetch_with_progress()?;
        let version = match self.version {
            Some(v) => match metadata.versions.iter().find(|ver| ver.name == v) {
                Some(ver) => ver,
                None => {
                    STDERR.write_line(&format!("Can't find version {} of the Android NDK.", v))?;
                    return Ok(());
                }
            },
            None => {
                &metadata.versions[Select::new()
                    .with_prompt("Version to install")
                    .items(
                        &metadata
                            .versions
                            .iter()
                            .map(|ver| &ver.name)
                            .collect::<Vec<&String>>(),
                    )
                    .default(0)
                    .interact_on(&*STDOUT)?]
            }
        };

        if version.download.checksum.is_none() {
            let continue_anyways =  !Confirmation::new()
                .with_text("This version doesn't have an associated checksum and won't be validated. Download it anyways?")
                .default(false)
                .interact_on(&*STDOUT)?;
            if !continue_anyways {
                return Ok(());
            }
        }

        let bin = version.download_with_progress()?;

        if !version.is_valid_with_progress(&bin) {
            STDERR.write_line("Download doesn't match checksum, aborting.")?;
            return Ok(());
        }

        let mut cursor = Cursor::new(bin);
        let zip = ZipArchive::new(&mut cursor)?;

        Ok(())
    }
}
