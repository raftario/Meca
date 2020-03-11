#[cfg(not(all(
    any(target_os = "windows", target_os = "macos", target_os = "linux"),
    target_pointer_width = "64"
)))]
compile_error!("Meca only supports 64 bit Windows, macOS and Linux.");

mod commands;
mod config;
mod globals;
mod ndk;

use crate::globals::STDERR;
use std::process;
use structopt::StructOpt;

type Result<T> = std::result::Result<T, anyhow::Error>;

#[derive(StructOpt, Debug)]
#[structopt(author, about)]
struct Opt {
    #[structopt(subcommand)]
    cmd: Command,
}

#[derive(StructOpt, Debug)]
pub enum Command {
    /// NDK related functionality
    Ndk(ndk::commands::Command),
}

impl Opt {
    fn run(self) -> Result<()> {
        match self.cmd {
            Command::Ndk(ndk) => ndk.run(),
        }
    }
}

fn main() {
    let opt = Opt::from_args();

    #[cfg(debug_assertions)]
    println!("{:#?}", opt);

    if let Err(e) = opt.run() {
        STDERR.write_line(&format!("{:#}", e)).unwrap_or_else(|_| {
            process::exit(-1);
        });
        process::exit(1);
    }
}
