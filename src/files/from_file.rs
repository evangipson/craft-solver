use logger::log_error;
use std::{fs, process::exit};

/// Responsible for populating any `struct` from a file.
pub trait FromFile {
    /// Populates `self` from `file_name`.
    fn populate(&self, file_name: &str) -> Self
    where
        Self: for<'de> serde::Deserialize<'de>,
    {
        let contents = match fs::read_to_string(file_name) {
            Ok(c) => c,
            Err(_) => {
                log_error!("Could not read file `{file_name}`");
                exit(1)
            }
        };
        toml::from_str(&contents).unwrap()
    }
}
