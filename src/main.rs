#[cfg(not(all(
    any(target_os = "windows", target_os = "macos", target_os = "linux"),
    target_pointer_width = "64"
)))]
compile_error!("Meca only supports 64 bit Windows, macOS and Linux.");

mod config;
mod globals;
mod ndk;

use structopt::StructOpt;

type Result<T> = std::result::Result<T, anyhow::Error>;

#[derive(StructOpt, Debug)]
struct Opt {}

fn main() {
    let opt = Opt::from_args();
    println!("{:#?}", opt);
}
