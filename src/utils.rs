use clap::ArgMatches;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub databases: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TaskSize {
    XXS,
    XS,
    S,
    M,
    L,
    XL,
    XXL,
}

impl TaskSize {
    pub fn to_int(&self) -> i32 {
        match self {
            TaskSize::XXS => 0,
            TaskSize::XS => 1,
            TaskSize::S => 2,
            TaskSize::M => 4,
            TaskSize::L => 5,
            TaskSize::XL => 6,
            TaskSize::XXL => 7,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            TaskSize::XXS => "XXS".to_string(),
            TaskSize::XS => "XS".to_string(),
            TaskSize::S => "S".to_string(),
            TaskSize::M => "M".to_string(),
            TaskSize::L => "L".to_string(),
            TaskSize::XL => "XL".to_string(),
            TaskSize::XXL => "XXL".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TaskTriage {
    Low,
    Medium,
    High,
    Urgent,
}

impl TaskTriage {
    pub fn to_int(&self) -> i32 {
        match self {
            TaskTriage::Low => 0,
            TaskTriage::Medium => 1,
            TaskTriage::High => 2,
            TaskTriage::Urgent => 3,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            TaskTriage::Low => "Low".to_string(),
            TaskTriage::Medium => "Medium".to_string(),
            TaskTriage::High => "High".to_string(),
            TaskTriage::Urgent => "Urgent".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TaskStatus {
    Todo,
    InProg,
    Done,
}

impl TaskStatus {
    pub fn to_int(&self) -> i32 {
        match self {
            TaskStatus::Todo => 0,
            TaskStatus::InProg => 1,
            TaskStatus::Done => 2,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            TaskStatus::Todo => "To-do".to_string(),
            TaskStatus::InProg => "In Progress".to_string(),
            TaskStatus::Done => "Done".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Task {
    pub name: String,
    pub triage: TaskTriage,
    pub status: TaskStatus,
    pub size: TaskSize,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TasksDatabase {
    pub name: String,
    pub tasks: Vec<Task>,
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
