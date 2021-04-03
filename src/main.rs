mod cli;
mod finder;

#[macro_use]
extern crate lazy_static;

use clap::crate_version;

fn main() {
    let version = crate_version!();
    cli::get_cli(version);
}
