//! Yet another minimalistic Rust-based task tracker.
//! Makes tasks by using CLI arguments or interactive prompts.
use crate::commands::{add::add, setup::setup};
use snowtracks::cli;
pub mod utils;
pub mod commands {
    pub mod add;
    pub mod setup;
}

fn main() {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("setup", sub_matches)) => {
            setup(sub_matches).unwrap();
        }
        Some(("add", sub_matches)) => {
            add(sub_matches).unwrap();
        }
        _ => unreachable!(),
    }
}
