use clap::{ArgMatches, Command, arg};
use json::{array, object};
use serde::{Deserialize, Serialize};
use std::{
    env,
    fs::{self, create_dir, exists},
    io::{self, Write},
};
use users::get_current_username;

#[derive(Serialize, Deserialize)]
struct Config {
    databases: Vec<String>,
}

/// Gets a string from arguments, defaulting to an empty String.
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
#[doc(hidden)]
pub fn get_from_args(matches: &ArgMatches, tgt: &str) -> Option<String> {
    return matches.get_one::<String>(tgt).map(|s| s.to_string());
}

/// Sets up database for task tracking
/// Name defaults to <username>_db
/// Path defaults to $XDG_DATA_HOME
/// Fails if path is not empty
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
pub fn setup(matches: &ArgMatches) -> io::Result<()> {
    let db_name = match get_from_args(matches, "name") {
        Some(arg) => arg,
        None => get_current_username()
            .and_then(|os_str| os_str.into_string().ok())
            .unwrap_or_else(|| "unknown".to_string()),
    };

    let db_dir = match get_from_args(matches, "path") {
        Some(arg) => arg,
        None => format!(
            "{}/snowtracks",
            env::var("XDG_DATA_HOME").unwrap_or("./snowtracks-db".to_string())
        ),
    };

    println!("==> Setting up database \"{}\" ({})", db_name, db_dir);

    if !exists(&db_dir).unwrap() {
        println!("[-] Creating database directory");
        create_dir(&db_dir).unwrap();
        print!("\x1b[An\x1b[2K");
        std::io::stdout().flush()?;
        println!("[+] Created database directory");
    } else {
        println!("[+] Database directory already exists");
    }

    let cfg_dir = match env::var("XDG_CONFIG_HOME") {
        Ok(path) => format!("{}/snowtracks", path),
        Err(_) => "./snowtracks-cfg".to_string(),
    };

    if !exists(&cfg_dir).unwrap() {
        println!("[-] Creating configuration directory");
        create_dir(&cfg_dir).unwrap();
        print!("\x1b[An\x1b[2K");
        std::io::stdout().flush()?;
        println!("[+] Created configuration directory");
    } else {
        println!("[+] Configuration directory already exists");
    }

    // TODO: Make DB and check if one already exists
    let db_task_file = format!("{}/{}.json", &cfg_dir, db_name);
    if !exists(&db_task_file)? {
        println!("[-] Creating database task file");
        fs::write(
            &db_task_file,
            object! {name: db_name.as_str(), tasks: array![]}.dump(),
        )
        .unwrap();
        print!("\x1b[An\x1b[2K");
        std::io::stdout().flush()?;
        println!("[+] Created database task file");
    } else {
        println!("[+] Database task file already exists");
    }

    let cfg_file = format!("{}/snowtracks.toml", cfg_dir);

    if !exists(&cfg_file).unwrap() {
        println!("[-] Generating base config ({})", &cfg_file);
        let cfg_obj: Config = Config {
            databases: vec![cfg_file.clone()],
        };

        fs::write(
            &cfg_file,
            toml::to_string_pretty(&cfg_obj).expect("Failed to parse Config struct!"),
        )
        .expect("Failed to write config!");

        print!("\x1b[An\x1b[2K");
        std::io::stdout().flush()?;
        println!("[+] Base config generated ({})", &cfg_file);
        return Ok(());
    }

    println!("[-] Checking existing config ({})", &cfg_file);
    let cfg_str = fs::read_to_string(&cfg_file)?;
    let mut cfg_obj: Config = toml::from_str(&cfg_str).expect("Failed to parse config file!");

    if cfg_obj.databases.contains(&db_dir) {
        print!("\x1b[An\x1b[2K");
        std::io::stdout().flush()?;
        println!("[+] Database already in config ({})", &db_dir);
        return Ok(());
    }

    print!("\x1b[An\x1b[2K");
    std::io::stdout().flush()?;
    println!("[-] Adding database to existing config ({})", &cfg_file);

    cfg_obj.databases.push(db_dir);
    fs::write(
        &cfg_file,
        toml::to_string_pretty(&cfg_obj).expect("Failed to parse new Config struct!"),
    )
    .expect("Failed to write new config!");

    print!("\x1b[An\x1b[2K");
    std::io::stdout().flush()?;
    println!("[+] New database added to config ({})", &cfg_file);

    Ok(())
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
    let _task_name = match get_from_args(matches, "name") {
        Some(arg) => arg,
        None => "".to_string(),
        // TODO: ^^^ Add interactive input here
        // as well as for other optional args
    };
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
                .about("Setup the snowtracks database")
                .arg(arg!(-n --name [NAME] "The name of the database"))
                .arg(arg!(-p --path [PATH] "The destination path of the database")),
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
