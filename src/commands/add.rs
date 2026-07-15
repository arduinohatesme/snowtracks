use crate::utils::{
    Config, Task, TaskSize, TaskStatus, TaskTriage, TasksDatabase, get_config_path, get_from_args,
    get_input_in,
};
use bincode_next::{self, config};
use clap::ArgMatches;
use sha1::{Digest, Sha1};
use std::{
    fs,
    io::{self, Write, stdout},
    str::FromStr,
};
use strum::IntoEnumIterator;

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
pub fn add(matches: &ArgMatches) -> io::Result<()> {
    let mut task = get_task_from_args(matches);

    println!("==> Adding task \"{}\"", task.name);
    println!("[-] Reading configuration");
    let cfg_str = fs::read_to_string(get_config_path()).unwrap();
    let cfg_obj: Config = toml::from_str(&cfg_str).expect("Failed to read config");
    let db_dir = cfg_obj.databases[0].to_string();
    let db_path = format!("{}/tasks.json", db_dir);

    print!("\x1b[An\r\x1b[2K");
    std::io::stdout().flush()?;
    println!("[+] Read configuration");
    println!("[i] Adding to database at {}", db_path);
    println!("[-] Reading database");

    let db_str = fs::read_to_string(&db_path).expect("Failed to read database");
    let mut db_obj: TasksDatabase =
        serde_json::from_str(&db_str).expect("Failed to cast database to TasksDatabase type");

    print!("\x1b[An\r\x1b[2K");
    std::io::stdout().flush()?;
    println!("[+] Read database");
    println!("[-] Generating task hash");

    let short_hash = generate_short_hash(&db_str);
    task.hash = Some(short_hash);

    print!("\x1b[An\r\x1b[2K");
    std::io::stdout().flush()?;
    println!("[+] Generated task hash");
    println!(
        "[i] Task hash is {}",
        task.hash.as_deref().unwrap_or("not found")
    );
    println!("[-] Adding task to database");

    db_obj.tasks.push(task);

    print!("\x1b[An\r\x1b[2K");
    std::io::stdout().flush()?;
    println!("[+] Added task to database");
    println!("[-] Writing new database");

    fs::write(
        &db_path,
        serde_json::to_string(&db_obj).expect("Failed to serialize string"),
    )
    .expect("Failed to write database");

    print!("\x1b[An\r\x1b[2K");
    std::io::stdout().flush()?;
    println!("[+] Wrote new database");
    println!("==> Added task \"{}\"", db_obj.tasks.last().unwrap().name);

    Ok(())
}

fn generate_short_hash(db_str: &str) -> String {
    let mut hasher = Sha1::new();
    hasher.update(&db_str.as_bytes());
    hasher.update(b"\0");
    hasher.update(
        bincode_next::encode_to_vec(&db_str, config::standard())
            .expect("Failed to encode database"),
    );

    let full_hash = hasher
        .finalize()
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect::<String>();

    let short_hash = full_hash[..7].to_string();
    short_hash
}

fn get_task_from_args(matches: &ArgMatches) -> Task {
    Task {
        name: get_name(matches),
        triage: get_triage(matches),
        status: get_status(matches),
        size: get_size(matches),
        hash: None,
    }
}

fn get_status(matches: &ArgMatches) -> TaskStatus {
    match get_from_args(matches, "progress") {
        Some(arg) => TaskStatus::from_str(&arg).unwrap_or({
            println!("Invalid task status.");
            print!("Enter task status (todo): ");
            stdout().flush().unwrap();

            let mut buf = String::new();
            let mut valid: Vec<String> = TaskStatus::iter().map(|t| t.to_string()).collect();
            valid.push("".to_string());

            get_input_in(&valid, &mut buf).expect("Failed to get task size level");

            if buf == "" {
                TaskStatus::Todo
            } else {
                TaskStatus::from_str(&buf.trim().to_uppercase()).unwrap()
            }
        }),
        None => {
            print!("Enter task status: ");
            stdout().flush().unwrap();

            let mut buf = String::new();
            let mut valid: Vec<String> = TaskStatus::iter().map(|t| t.to_string()).collect();
            valid.push("".to_string());

            get_input_in(&valid, &mut buf).expect("Failed to get task status");

            if buf == "" {
                TaskStatus::Todo
            } else {
                TaskStatus::from_str(&buf.trim().to_lowercase()).unwrap()
            }
        }
    }
}

fn get_size(matches: &ArgMatches) -> TaskSize {
    match get_from_args(matches, "size") {
        Some(arg) => TaskSize::from_str(&arg).unwrap_or({
            println!("Invalid task size.");
            print!("Enter task size: ");
            stdout().flush().unwrap();

            let mut buf = String::new();
            let valid: Vec<String> = TaskSize::iter().map(|t| t.to_string()).collect();

            get_input_in(&valid, &mut buf).expect("Failed to get task size");
            TaskSize::from_str(&buf.trim().to_lowercase()).unwrap()
        }),
        None => {
            print!("Enter task size: ");
            stdout().flush().unwrap();

            let mut buf = String::new();
            let valid: Vec<String> = TaskSize::iter()
                .map(|t| t.to_string().to_lowercase())
                .collect();

            get_input_in(&valid, &mut buf).expect("Failed to get task size");
            TaskSize::from_str(&buf.trim().to_lowercase()).unwrap()
        }
    }
}

fn get_name(matches: &ArgMatches) -> String {
    match get_from_args(matches, "name") {
        Some(arg) => arg,
        None => {
            print!("Enter task name: ");
            stdout().flush().unwrap();
            let mut buf = String::new();
            io::stdin()
                .read_line(&mut buf)
                .expect("Failed to read task name input");
            buf.trim().to_string()
        }
    }
}

fn get_triage(matches: &ArgMatches) -> TaskTriage {
    match get_from_args(matches, "triage") {
        Some(arg) => TaskTriage::from_str(&arg).unwrap_or({
            println!("Invalid triage level.");
            print!("Enter triage level: ");
            stdout().flush().unwrap();

            let mut buf = String::new();
            let valid: Vec<String> = TaskTriage::iter().map(|t| t.to_string()).collect();

            get_input_in(&valid, &mut buf).expect("Failed to get task triage level");
            TaskTriage::from_str(&buf).unwrap()
        }),
        None => {
            print!("Enter triage level: ");
            stdout().flush().unwrap();

            let mut buf = String::new();
            let valid: Vec<String> = TaskTriage::iter().map(|t| t.to_string()).collect();

            get_input_in(&valid, &mut buf).expect("Failed to get task triage level");
            TaskTriage::from_str(&buf.trim().to_lowercase()).unwrap()
        }
    }
}
