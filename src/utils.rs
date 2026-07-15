use clap::ArgMatches;
use dialoguer::{Confirm, Select};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::env;
use strum::{Display, EnumString, FromRepr, VariantNames};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub databases: Vec<String>,
}

#[derive(Serialize_repr, Deserialize_repr, EnumString, FromRepr, VariantNames, Display, Debug)]
#[serde(rename_all = "camelCase")]
#[strum(serialize_all = "lowercase")]
#[repr(i32)]
pub enum TaskSize {
    XXS = 0,
    XS = 1,
    S = 3,
    M = 4,
    L = 5,
    XL = 6,
    XXL = 7,
}

#[derive(Serialize_repr, Deserialize_repr, EnumString, FromRepr, VariantNames, Display, Debug)]
#[serde(rename_all = "camelCase")]
#[strum(serialize_all = "lowercase")]
#[repr(i32)]
pub enum TaskTriage {
    Low = 0,
    Medium = 1,
    High = 2,
    Urgent = 3,
}

#[derive(Serialize_repr, Deserialize_repr, EnumString, FromRepr, VariantNames, Display, Debug)]
#[serde(rename_all = "camelCase")]
#[strum(serialize_all = "lowercase")]
#[repr(i32)]
pub enum TaskStatus {
    Todo = 0,
    InProg = 1,
    Done = 2,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Task {
    pub name: String,
    pub triage: TaskTriage,
    pub status: TaskStatus,
    pub size: TaskSize,
    pub hash: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TasksDatabase {
    pub name: String,
    pub tasks: Vec<Task>,
}

pub fn get_confirmation(prompt: &str) -> Option<bool> {
    Confirm::new()
        .with_prompt(prompt)
        .interact_opt()
        .expect("Failed to get confirmation")
}

pub fn get_input_in(prompt: &str, possible: &[&str]) -> Option<i32> {
    if possible.is_empty() {
        return None;
    }

    let labels: Vec<String> = possible.iter().map(|item| item.to_string()).collect();
    let selection = Select::new()
        .with_prompt(prompt)
        .items(&labels)
        .default(0)
        .interact_opt()
        .ok()?;

    selection.map(|idx| idx as i32)
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

pub fn get_config_path() -> String {
    format!(
        "{}/snowtracks/snowtracks.toml",
        env::var("XDG_CONFIG_HOME").expect("Failed to read config var")
    )
}
