use crate::utils::{get_database_object, get_database_path, get_from_args};
use clap::ArgMatches;
use std::{fs, io};

pub fn delete(matches: &ArgMatches) -> io::Result<()> {
    let hash = get_from_args(matches, "HASH").expect("Failed to get hash from arguments");
    for db_dir in fs::read_dir(get_database_path(None))? {
        let db_file = format!(
            "{}/tasks.json",
            db_dir?
                .path()
                .to_str()
                .expect("Failed to get database directory path")
        );
        let db_obj = get_database_object(&db_file);
        let mut tmp_db_obj = db_obj.clone();

        tmp_db_obj.tasks.retain(|t| {
            !t.hash
                .clone()
                .unwrap_or("zzz".to_string())
                .starts_with(&hash)
        });

        if tmp_db_obj.tasks.len() < db_obj.tasks.len() - 1 {
            panic!("Hash too ambiguous");
        }

        if tmp_db_obj.tasks.len() == db_obj.tasks.len() {
            panic!("Hash did not match any tasks");
        }

        fs::write(
            &db_file,
            serde_json::to_string_pretty(&db_obj).expect("Failed to serialize database"),
        )
        .expect("Failed to write database");
    }
    Ok(())
}
