//! Yet another minimalistic Rust-based task tracker.
//! Makes tasks by using CLI arguments or interactive prompts.
use snowtracks::{add, cli, setup};

fn main() {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("setup", sub_matches)) => {
            setup(sub_matches);
        }
        Some(("add", sub_matches)) => {
            add(sub_matches);
        }
        _ => unreachable!(),
    }
}
