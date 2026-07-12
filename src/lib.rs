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
/// let matches = my_cli().try_get_matches_from(vec!["my_command", "John Smith"]).unwrap();
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
/// # use clap::{ArgMatches, Command, arg};
/// # use snowtracks::{cli, get_from_args, setup};
///
/// let matches = cli().try_get_matches_from(vec!["snow", "setup"]).unwrap();
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
    let task_name = get_from_args(matches, "name");
    if task_name != "" {
        println!("The first argument was {}", task_name)
    }
}

/// The main clap entry point for the snowtracks CLI
///
/// # Examples
///
/// ```
/// # use clap::{ArgMatches, Command, arg};
/// # use snowtracks::cli;
///
/// let setup_matches = cli().try_get_matches_from(vec!["snow", "setup"]).unwrap();
/// let setup_with_name_matches = cli()
///     .try_get_matches_from(vec!["snow", "setup", "--name", "your_db"])
///     .unwrap();
///
/// let (sub_command, sub_matches) = setup_matches.subcommand().unwrap();
/// let (sub_with_name_command, sub_with_name_matches) =
///     setup_with_name_matches.subcommand().unwrap();
///
/// assert_eq!(sub_command, "setup");
/// assert_eq!(sub_with_name_command, "setup");
/// assert!(sub_matches.get_one::<String>("name").is_none());
/// assert_eq!(
///     sub_with_name_matches.get_one::<String>("name").unwrap(),
///     "your_db"
/// );
/// ```
///
/// ```
/// # use clap::{ArgMatches, Command, arg};
/// # use snowtracks::cli;
///
/// let add_matches = cli().try_get_matches_from(vec!["snow", "add"]).unwrap();
/// let add_with_params_matches = cli()
///     .try_get_matches_from(vec![
///         "snow", "add", "-n", "My
/// Task", "-t", "medium", "-p", "done",
///     ])
///     .unwrap();
///
/// let (sub_command, sub_matches) = add_matches.subcommand().unwrap();
/// let (sub_with_params_command, sub_with_params_matches) =
///     add_with_params_matches.subcommand().unwrap();
///
/// assert_eq!(sub_command, "add");
/// assert_eq!(sub_with_params_command, "add");
/// assert!(sub_matches.get_one::<String>("triage").is_none());
/// assert_eq!(
///     sub_with_params_matches.get_one::<String>("triage").unwrap(),
///     "medium"
/// );
/// ```
///
/// ```
/// # use clap::{ArgMatches, Command, arg};
/// # use snowtracks::cli;
///
/// let delete_matches = cli()
///     .try_get_matches_from(vec!["snow", "delete", "42"])
///     .unwrap();
///
/// let (sub_command, sub_matches) = delete_matches.subcommand().unwrap();
///
/// assert_eq!(sub_command, "delete");
/// assert_eq!(sub_matches.get_one::<String>("ID").unwrap(), "42");
/// ```
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
