use crate::{globals::STDOUT, ndk::metadata::Metadata, Result};
use indicatif::ProgressBar;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct Command {}

impl Command {
    pub fn run(self) -> Result<()> {
        let p = ProgressBar::new_spinner();
        p.set_message("Fetching metadata");
        let metadata = Metadata::fetch()?;
        p.finish_and_clear();

        STDOUT.write_line(
            &metadata
                .versions
                .keys()
                .map(|s| s.as_str().to_owned())
                .collect::<Vec<String>>()
                .join("\n"),
        )?;
        Ok(())
    }
}
