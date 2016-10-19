
//! primary crate for iron_with_db
//! 
//! ## Overview
//! 
//! Iron with db implements the iron webserver with a flexible db backend that
//! allows for easy mvc development.
//!
//! The immediate goal is to have a fully functional mongodb backend.
//!
//! In the future I will implement more dbs with common syntax for calls.
//!
//! - zpallin
//!

extern crate iron;
extern crate router;
extern crate persistent;

pub use iron::prelude::*;
pub use iron::{BeforeMiddleware, AfterMiddleware, status};
pub use iron::typemap::Key;
pub use router::Router;
pub use persistent::{Read};

/// Struct used to pass numerous configuration values into the request chain
pub struct AppConfig {
    /// stores the application name
    pub name: String, 

    /// stores the logfile path
    pub logfile_path: String,

    /// stores the url to the db
    pub database_url: String,
}

impl AppConfig {

    /// creates a new AppConfig
    pub fn new(name: &str, logfile_path: &str, database_url: &str) -> AppConfig {

        AppConfig {
            name: String::from(name),
            logfile_path: String::from(logfile_path),
            database_url: String::from(database_url),
        }
    }

    /// builder pattern for assigning name over current values
    pub fn set_name(&mut self, name: &str) -> &mut AppConfig {

        self.name = String::from(name);
        self
    }

    /// builder pattern for assigning logfile path over current values
    pub fn set_logfile_path(&mut self, logfile_path: &str) -> &mut AppConfig {

        self.logfile_path = String::from(logfile_path);
        self
    }

    /// builder pattern for assigning database url over current values
    pub fn set_database_url(&mut self, database_url: &str) -> &mut AppConfig {

        self.database_url = String::from(database_url);
        self
    }

    /// for finalizing builder transactions
    pub fn finalize(&self) -> AppConfig {

        AppConfig {
            name: self.name.clone(),
            logfile_path: self.logfile_path.clone(),
            database_url: self.database_url.clone(),
        }
    }

    /// debug for printing out config just in case
    pub fn display(&self) {

        println!(
            "ServerData: {{\n    \
            name: {}\n    \
            logfile_path: {},\n    \
            database_url: {},\n\
            }}",
            self.name,
            self.logfile_path,
            self.database_url);
    }
}

impl Default for AppConfig {

    /// setting default values for AppConfig
    fn default() -> Self {

        AppConfig {
            name: String::from("iron_with_db"),
            logfile_path: format!("{:?}/log.txt", std::env::current_dir().unwrap()),
            database_url: String::from("localhost:27017/iron_with_db"),
        }
    }
}

/// struct for containing server data
#[derive(Copy, Clone)]
pub struct AppConfigKey;

impl Key for AppConfigKey {
    type Value = AppConfig;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
