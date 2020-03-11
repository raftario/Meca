mod add;
mod install;
mod list;
mod remove;
mod select;

// Passthrough
mod build;
mod gdb;
mod stack;
mod which;

use crate::Result;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct Command {
    #[structopt(subcommand)]
    cmd: SubCommand,
}

#[derive(StructOpt, Debug)]
pub enum SubCommand {
    /// Installs a new NDK
    Install(install::Command),
    /// Lists available and/or installed NDKs
    List(list::Command),
}

impl Command {
    pub fn run(self) -> Result<()> {
        match self.cmd {
            SubCommand::Install(install) => install.run(),
            SubCommand::List(list) => list.run(),
        }
    }
}
