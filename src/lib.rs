use clap::{ArgMatches, Command, arg};

/// Gets a string from arguments, defaulting to empty.
///
/// # Arguments
///
/// * `matches` - The ArgMatches to match
/// * `tgt` - The name of the arg to get
///
/// # Examples
///
/// ```
/// use clap::{Command, arg};
/// use snowtracks::get_from_args;
///
/// fn my_cli() -> Command {
///    Command::new("my_command")
///        .arg(arg!(<NAME> "Your name"))
/// }
///
/// let matches = my_cli().get_matches_from(vec!["my_command", "John Smith"]);
/// let arg = get_from_args(&matches, "NAME");
/// assert_eq!(arg, "John Smith".to_string());
/// ```
pub fn get_from_args(matches: &ArgMatches, tgt: &str) -> String {
    return matches
        .get_one::<String>(tgt)
        .map(|s| s.to_string())
        .unwrap_or_else(|| "".to_string());
}

/// Sets up database for task tracking
///
/// # Arguments
///
/// * `matches` - Clap CLI ArgMatches to run with
///
/// # Examples
///
/// ```
/// use clap::{ArgMatches, Command, arg};
/// use snowtracks::{cli, get_from_args, setup};
///
/// let matches = cli().get_matches_from(vec!["snow", "setup"]);
/// if let Some(sub_matches) = matches.subcommand_matches("setup") {
///     setup(&sub_matches);
/// } else {
///     panic!("Error: Setup subcommand not triggered.");
/// }
///
/// let matches_with_name = cli().get_matches_from(vec!["snow", "setup", "--name", "my_db"]);
/// if let Some(sub_matches_with_name) = matches_with_name.subcommand_matches("setup") {
///     setup(&sub_matches_with_name);
/// } else {
///     panic!("Error: Setup subcommand not triggered.");
/// }
/// ```
pub fn setup(matches: &ArgMatches) {
    let db_name = get_from_args(matches, "name");
    if db_name != "" {
        println!("Setting up database named {}", db_name)
    }
}

/// Adds a task to the database
///
/// # Arguments
///
/// * `matches` - Clap CLI ArgMatches to run with
///
/// # Examples
///
/// ```
/// use clap::{ArgMatches, Command, arg};
/// use snowtracks::{add, cli, get_from_args};
///
/// let matches = cli().get_matches_from(vec!["snow", "add"]);
/// if let Some(sub_matches) = matches.subcommand_matches("add") {
///     add(&sub_matches);
/// } else {
///     panic!("Error: Add subcommand not triggered.");
/// }
/// ```
pub fn add(matches: &ArgMatches) {
    let task_name = get_from_args(matches, "name");
    if task_name != "" {
        println!("The first argument was {}", task_name)
    }
}

pub fn cli() -> Command {
    Command::new("snow")
        .about("Yet another rust-based task tracker")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .subcommand(
            Command::new("setup")
                .about("Setup the snowtracks database")
                .arg(arg!(-n --name [NAME] "The name of the database")),
        )
        .subcommand(
            Command::new("add")
                .about("Add a task to the tracker")
                .arg(arg!(-n --name [NAME] "The name of the task"))
                .arg(arg!(-t --triage [TRIAGE] "The triage level of the task"))
                .arg(arg!(-p --progress [PROGRESS] "The progress level of the task")),
        )
        .subcommand(
            Command::new("delete")
                .about("Remove a task from the tracker")
                .arg(arg!(<ID> "The ID number of the task")),
        )
}
