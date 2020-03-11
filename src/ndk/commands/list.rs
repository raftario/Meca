use crate::{config::Config, globals::STDOUT, ndk::metadata::Metadata, Result};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct Command {
    #[structopt(short = "i", long = "installed")]
    /// Displays only installed versions
    installed: bool,
}

impl Command {
    pub fn run(self) -> Result<()> {
        let installed: Vec<String> = Config::read()?
            .ndk
            .installed
            .into_iter()
            .map(|i| i.name)
            .collect();
        if self.installed {
            STDOUT.write_line(&installed.join("\n"))?;
            return Ok(());
        }

        let metadata = Metadata::fetch_with_progress()?;

        STDOUT.write_line(
            &metadata
                .versions
                .into_iter()
                .map(|v| {
                    if installed.iter().any(|i| i == &v.name) {
                        format!("{} (installed)", v.name)
                    } else {
                        v.name
                    }
                })
                .collect::<Vec<String>>()
                .join("\n"),
        )?;
        Ok(())
    }
}
