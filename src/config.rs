/*
 * The config module is responsible for deserializing the yaml configuration
 */
use serde::Deserialize;
use std::error::Error;
use std::io::BufReader;
use std::fs::File;
use std::path::Path;

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    pub shares: Vec<Share>,
}

impl Config {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Config, Box<dyn Error>> {
        let file = File::open(path)?;
        let c = serde_yaml::from_reader(BufReader::new(file))?;

        Ok(c)
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct Share {
    pub name: String,
    pub schemas: Vec<Schema>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Schema {
    pub name: String,
    pub tables: Vec<Table>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Table {
    pub name: String,
    pub location: String,
}