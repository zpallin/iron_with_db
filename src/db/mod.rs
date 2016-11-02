
//! db module for iron_with_db
//!
//! ## Overview
//!
//! This module provides some tools for handling configuration input, 
//! designed primarily with databases in mind.
//!
//! The DatabaseConfig struct, for example, is designed to accept a db conf
//! string and then split it up for your use case.
//!
//! - zpallin
//!

use prelude::*;

#[derive(Debug)]
pub struct DatabaseConfig {

    /// name of the database in the server
    pub dbname: String,

    /// hostname
    pub hostname: String,

    /// port
    pub port: u16,
}

impl DatabaseConfig {

    pub fn new(database_url: &str) -> DatabaseConfig {

        // get the dbname
        let split = database_url.split("/").collect::<Vec<&str>>();
        let dbname = split[1].to_owned();

        // get the hostname
        let split = split[0].split(":").collect::<Vec<&str>>();
        let hostname = split[0].to_owned();

        // get the port
        let port: u16 = split[1].to_owned().parse().ok().expect("DbConfig: Wanted a number");

        DatabaseConfig
        {
            dbname: dbname,
            hostname: hostname,
            port: port,
        }
    }

    pub fn display(&self) -> String {

        format!(
            "DatabaseConfig {{\n    \
            dbname: {},\n    \
            hostname: {},\n    \
            port: {},\n\
            }}",
            self.dbname,
            self.hostname,
            self.port,
        )
    }
}

#[derive(Copy, Clone)]
pub struct DatabaseConfigKey;

impl Key for DatabaseConfigKey {
    type Value = DatabaseConfig;
}

pub mod tests;

