#[cfg(not(all(
    any(target_os = "windows", target_os = "macos", target_os = "linux"),
    target_pointer_width = "64"
)))]
compile_error!("Meca only supports 64 bit Windows, macOS and Linux.");

mod commands;
mod config;
mod globals;
mod ndk;

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

fn main() {
    let opt = Opt::from_args();
    println!("{:#?}", opt);
}
