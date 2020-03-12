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
        let config = Config::read()?;
        let selected = config.ndk.selected;
        let installed: Vec<String> = config.ndk.installs.into_iter().map(|i| i.name).collect();

        if self.installed {
            STDOUT.write_line(
                &installed
                    .into_iter()
                    .map(|s| {
                        if selected.as_ref() == Some(&s) {
                            format!("{} (selected)", s)
                        } else {
                            s
                        }
                    })
                    .collect::<Vec<String>>()
                    .join("\n"),
            )?;
            return Ok(());
        }

        let metadata = Metadata::fetch_with_progress()?;

        STDOUT.write_line(
            &metadata
                .versions
                .into_iter()
                .map(|v| {
                    if selected.as_ref() == Some(&v.name) {
                        format!("{} (selected)", v.name)
                    } else if installed.iter().any(|i| i == &v.name) {
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
