use serde::{Deserialize, Serialize};
use std::fs;
use toml::ser::Error;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub(crate) database: DatabaseConfig,
}

#[derive(Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub(crate) connection_string: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            database: DatabaseConfig {
                connection_string: "postgres://username:password@host/database".to_string(),
            },
        }
    }
}

const CONFIG_PATH: &'static str = "config.toml";

pub fn load_config() -> Result<Config, Error> {
    if !fs::exists(CONFIG_PATH.to_string()).unwrap() {
        let config = Config::default();
        fs::write(CONFIG_PATH.to_string(), toml::to_string_pretty(&config)?)
            .expect("Unable to create config file");
    }
    let content = fs::read_to_string(CONFIG_PATH.to_string()).expect("Could not read config file");
    let config: Config = toml::from_str(&content).expect("Cannot parse config");

    Ok(config)
}
