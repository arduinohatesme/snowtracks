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
    time::SystemTime,
};
use strum::VariantNames;

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

    generate_short_hash(&db_str, &mut task);

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
    println!(
        "==> Added task \"{}\" with hash {} to database \"{}\"",
        db_obj.tasks.last().unwrap().name,
        db_obj
            .tasks
            .last()
            .expect("Failed to get last task")
            .hash
            .as_ref()
            .unwrap_or(&"not found".to_string()),
        db_obj.name
    );

    Ok(())
}

fn generate_short_hash(db_str: &str, task: &mut Task) {
    let mut hasher = Sha1::new();
    hasher.update(&db_str.as_bytes());
    hasher.update(b"\0");
    hasher.update(
        bincode_next::serde::encode_to_vec(&task, config::standard())
            .expect("Failed to encode database"),
    );
    hasher.update(b"\0");
    hasher.update(
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("Failed to get system time")
            .as_micros()
            .to_be_bytes(),
    );

    let full_hash = hasher
        .finalize()
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect::<String>();

    task.hash = Some(full_hash[..6].to_string());
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
            println!("Invalid task status");
            let valid = TaskStatus::VARIANTS;

            let selection_int =
                get_input_in("Enter task status", &valid).expect("Failed to get task status");

            TaskStatus::from_repr(selection_int).expect("Failed to convert selection to TaskStatus")
        }),
        None => {
            let valid = TaskStatus::VARIANTS;

            let selection_int =
                get_input_in("Enter task status", &valid).expect("Failed to get task status");

            TaskStatus::from_repr(selection_int).expect("Failed to convert selection to TaskStatus")
        }
    }
}

fn get_size(matches: &ArgMatches) -> TaskSize {
    match get_from_args(matches, "size") {
        Some(arg) => TaskSize::from_str(&arg).unwrap_or({
            println!("Invalid task size");
            let valid = TaskSize::VARIANTS;

            let selection_int =
                get_input_in("Enter task size", &valid).expect("Failed to get task size");

            TaskSize::from_repr(selection_int).expect("Failed to convert selection to TaskSize")
        }),
        None => {
            let valid = TaskSize::VARIANTS;

            let selection_int =
                get_input_in("Enter task size", &valid).expect("Failed to get task size");

            TaskSize::from_repr(selection_int).expect("Failed to convert selection to TaskSize")
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
            stdout().flush().unwrap();

            let valid = TaskTriage::VARIANTS;

            let selection_int = get_input_in("Enter triage level", &valid)
                .expect("Failed to get task triage level");

            TaskTriage::from_repr(selection_int).expect("Failed to convert selection to TaskTriage")
        }),
        None => {
            let valid = TaskTriage::VARIANTS;

            let selection_int = get_input_in("Enter triage level", &valid)
                .expect("Failed to get task triage level");

            TaskTriage::from_repr(selection_int).expect("Failed to convert selection to TaskTriage")
        }
    }
}
