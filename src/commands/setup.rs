use crate::utils::{Config, TasksDatabase, get_from_args};
use clap::ArgMatches;
use std::env;
use std::sync::OnceLock;
use std::{
    fs::{self, create_dir, exists},
    io::{self, Write},
};
use users::get_current_username;

static CONFIG_FILE_PATH: OnceLock<String> = OnceLock::new();

fn get_input_in(possible: Vec<&str>, buf: &mut String) -> io::Result<()> {
    let mut ran_before = false;
    loop {
        if ran_before {
            print!("\x1b[An\r\x1b[2K");
            print!("Invalid input. Try again: ");
            io::stdout().flush()?;
        }
        buf.clear();
        io::stdin().read_line(buf)?;

        let clean_buf = buf.trim().to_lowercase();
        ran_before = true;

        if possible.contains(&clean_buf.as_str()) {
            break;
        }
    }

    print!("\x1b[An\r\x1b[2K");
    print!("\x1b[An\r\x1b[2K");
    Ok(())
}

/// Validates database tasks
/// Returns Err if invalid
///
/// # Invalid Cases
/// * File does not exist
/// * File is not valid JSON
/// * File JSON cannot be casted to TasksDatabase
///
/// # Arguments
///
/// * `file_path` - File path to check
fn validate_tasks_file(file_path: &str) -> io::Result<()> {
    let db_contents_str = fs::read_to_string(&file_path)?;
    let _db_contents: TasksDatabase = serde_json::from_str(&db_contents_str)?;

    Ok(())
}

/// Removes a database from the user's config.
///
/// # Arguments
///
/// * `db_path` - The path to the database to remove
fn remove_database_from_config(db_path: &str) -> io::Result<()> {
    let cfg_str = fs::read_to_string(CONFIG_FILE_PATH.get().unwrap())?;
    let mut cfg_obj: Config = toml::from_str(&cfg_str).expect("Failed to parse config file");

    cfg_obj.databases.retain(|d| d != db_path);
    fs::write(
        CONFIG_FILE_PATH.get().unwrap(),
        toml::to_string_pretty(&cfg_obj).expect("Failed to parse new Config struct"),
    )
    .expect("Failed to write new config");
    Ok(())
}

/// Prompts the user if they want to remove a database
/// from their config, and does so if they do.
///
/// # Arguments
///
/// * `db_path` - The path to the database in question
fn prompt_remove_database_from_config(db_path: &str) -> io::Result<()> {
    print!("Do you want to remove {} from your config? (Y/n) ", db_path);
    io::stdout().flush()?;

    let mut buf = "".to_string();
    get_input_in(vec!["y", "n", ""], &mut buf).unwrap();
    buf = buf.trim().to_lowercase();

    if buf == "n" {
        return Ok(());
    }

    remove_database_from_config(db_path)?;
    Ok(())
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
        print!("\x1b[An\r\x1b[2K");
        std::io::stdout().flush()?;
        println!("[+] Created database directory");
    } else {
        println!("[+] Database directory already exists");
    }

    let db_task_file = format!("{}/tasks.json", &db_dir);
    if !exists(&db_task_file)? {
        println!("[-] Creating database task file");

        let base_db = TasksDatabase {
            name: db_name.clone(),
            tasks: vec![],
        };
        fs::write(
            &db_task_file,
            serde_json::to_string(&base_db).expect("Failed to serialize base tasks file"),
        )
        .unwrap();

        print!("\x1b[An\r\x1b[2K");
        std::io::stdout().flush()?;
        println!("[+] Created database task file");
    } else {
        println!("[+] Database task file already exists");
    }

    let cfg_dir = match env::var("XDG_CONFIG_HOME") {
        Ok(path) => format!("{}/snowtracks", path),
        Err(_) => "./snowtracks-cfg".to_string(),
    };

    if CONFIG_FILE_PATH.get().is_none() {
        CONFIG_FILE_PATH
            .set(format!("{}/snowtracks.toml", cfg_dir))
            .expect("Failed to set config file path");
    }

    if !exists(&cfg_dir).unwrap() {
        println!("[-] Creating configuration directory");
        create_dir(&cfg_dir).unwrap();
        print!("\x1b[An\r\x1b[2K");
        std::io::stdout().flush()?;
        println!("[+] Created configuration directory");
    } else {
        println!("[+] Configuration directory already exists");
    }

    if !exists(CONFIG_FILE_PATH.get().unwrap()).unwrap() {
        println!("[-] Generating base config");
        let cfg_obj: Config = Config {
            databases: vec![db_dir.clone()],
        };

        fs::write(
            CONFIG_FILE_PATH.get().unwrap(),
            toml::to_string_pretty(&cfg_obj).expect("Failed to parse Config struct"),
        )
        .expect("Failed to write config");

        print!("\x1b[An\r\x1b[2K");
        std::io::stdout().flush()?;
        println!("[+] Base config generated");
        return Ok(());
    }

    let cfg_str = fs::read_to_string(CONFIG_FILE_PATH.get().unwrap())?;
    let mut cfg_obj: Config = toml::from_str(&cfg_str).expect("Failed to parse config file");

    if !cfg_obj.databases.contains(&db_dir) {
        print!("\x1b[An\r\x1b[2K");
        std::io::stdout().flush()?;
        println!(
            "[-] Adding database to existing config ({})",
            CONFIG_FILE_PATH.get().unwrap()
        );

        cfg_obj.databases.push(db_dir);
        fs::write(
            CONFIG_FILE_PATH.get().unwrap(),
            toml::to_string_pretty(&cfg_obj).expect("Failed to parse new Config struct"),
        )
        .expect("Failed to write new config");

        print!("\x1b[An\r\x1b[2K");
        std::io::stdout().flush()?;
        println!(
            "[+] New database added to config ({})",
            CONFIG_FILE_PATH.get().unwrap()
        );
    } else {
        print!("\x1b[An\r\x1b[2K");
        std::io::stdout().flush()?;
        println!("[+] Database already in config ({})", &db_dir);
    }

    println!("[-] Validating config");

    for db in &cfg_obj.databases {
        let tasks_path = format!("{}/tasks.json", db);
        match validate_tasks_file(&tasks_path) {
            Ok(_) => {}
            Err(e) => {
                println!("Failed to read database file {}: {}", tasks_path, e);
                prompt_remove_database_from_config(&db.trim())?;
            }
        }
    }

    print!("\x1b[An\r\x1b[2K");
    std::io::stdout().flush()?;
    println!("[+] Validated config");
    println!("==> Finished setup!");

    Ok(())
}
