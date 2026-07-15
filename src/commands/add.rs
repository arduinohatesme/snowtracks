use crate::utils::{
    Task, TaskSize, TaskStatus, TaskTriage, get_config_path, get_from_args, get_input_in,
};
use clap::ArgMatches;
use std::{
    fs::read_to_string,
    io::{self, Write, stdout},
    str::FromStr,
};
use strum::IntoEnumIterator;

fn get_task_from_args(matches: &ArgMatches) -> Task {
    let task_name: String = match get_from_args(matches, "name") {
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
    };

    let task_triage: TaskTriage = match get_from_args(matches, "triage") {
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
    };

    let task_size: TaskSize = match get_from_args(matches, "size") {
        Some(arg) => TaskSize::from_str(&arg).unwrap_or({
            println!("Invalid task size.");
            print!("Enter task size: ");
            stdout().flush().unwrap();

            let mut buf = String::new();
            let valid: Vec<String> = TaskSize::iter().map(|t| t.to_string()).collect();

            get_input_in(&valid, &mut buf).expect("Failed to get task size level");
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
    };

    let task_status: TaskStatus = match get_from_args(matches, "progress") {
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
            print!("Enter task size: ");
            stdout().flush().unwrap();

            let mut buf = String::new();
            let mut valid: Vec<String> = TaskStatus::iter().map(|t| t.to_string()).collect();
            valid.push("".to_string());

            get_input_in(&valid, &mut buf).expect("Failed to get task size");

            if buf == "" {
                TaskStatus::Todo
            } else {
                TaskStatus::from_str(&buf.trim().to_lowercase()).unwrap()
            }
        }
    };

    Task {
        name: task_name,
        triage: task_triage,
        status: task_status,
        size: task_size,
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
    let task = get_task_from_args(matches);
    read_to_string(get_config_path()).unwrap();
    println!("Made task: {:#?}", task);
}
