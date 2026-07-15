use clap::ArgMatches;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::{
    env,
    io::{self, Write},
};
use strum::{Display, EnumIter, EnumString, FromRepr};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub databases: Vec<String>,
}

#[derive(Serialize_repr, Deserialize_repr, EnumString, FromRepr, EnumIter, Display, Debug)]
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

#[derive(Serialize_repr, Deserialize_repr, EnumString, FromRepr, EnumIter, Display, Debug)]
#[serde(rename_all = "camelCase")]
#[strum(serialize_all = "lowercase")]
#[repr(i32)]
pub enum TaskTriage {
    Low = 0,
    Medium = 1,
    High = 2,
    Urgent = 3,
}

#[derive(Serialize_repr, Deserialize_repr, EnumString, FromRepr, EnumIter, Display, Debug)]
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

pub fn get_input_in(possible: &[String], buf: &mut String) -> io::Result<()> {
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

        if possible.contains(&clean_buf) {
            *buf = clean_buf;
            break;
        }
    }
    Ok(())
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
