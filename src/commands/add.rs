use crate::utils::{TaskTriage, get_from_args};
use clap::ArgMatches;
use std::io;

/// Adds a task to the database
///
/// # Arguments
///
/// * `matches` - Clap CLI ArgMatches to run with
///
/// # Examples
///
/// ```
/// # use clap::{ArgMatches, Command, arg};
/// # use snowtracks::{add, cli, get_from_args};
///
/// let matches = cli().try_get_matches_from(vec!["snow", "add"]).unwrap();
/// if let Some(sub_matches) = matches.subcommand_matches("add") {
///     add(&sub_matches);
/// } else {
///     panic!("Error: Add subcommand not triggered.");
/// }
/// ```
pub fn add(matches: &ArgMatches) {
    let task_name: String = match get_from_args(matches, "name") {
        Some(arg) => arg,
        None => {
            println!("Enter task name: ");
            let mut buf = String::new();
            io::stdin()
                .read_line(&mut buf)
                .expect("Failed to read task name input");
            buf.trim().to_string()
        }
    };

    let task_triage: TaskTriage;
}
