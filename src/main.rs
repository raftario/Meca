#[cfg(not(all(
    any(target_os = "windows", target_os = "macos", target_os = "linux"),
    target_pointer_width = "64"
)))]
compile_error!("Meca only supports 64 bit Windows, macOS and Linux.");

mod config;
mod globals;
mod metadata;

fn main() {
    println!("Hello, world!");
}
