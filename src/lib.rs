use clap::{Command, arg};

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
///         "snow",
///         "add",
///         "-n",
///         "The task at hand",
///         "-t",
///         "medium",
///         "-p",
///         "done",
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
        .about("Don't lose yourself in a blizzard of thoughts.")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .subcommand(
            Command::new("setup")
                .about("Setup a snowtracks database")
                .arg(arg!(-n --name [NAME] "The name of the database"))
                .arg(arg!(-p --path [PATH] "The destination path of the database")),
        )
        .subcommand(
            Command::new("add")
                .about("Add a task to a tracker")
                .arg(arg!(-n --name [NAME] "The name of the task"))
                .arg(arg!(-t --triage [TRIAGE] "The triage level of the task"))
                .arg(arg!(-s --size [SIZE] "The size of the task"))
                .arg(arg!(-p --progress [PROGRESS] "The progress level of the task")),
        )
        .subcommand(
            Command::new("delete")
                .about("Remove a task from the tracker")
                .arg(arg!(<ID> "The ID number of the task")),
        )
}
