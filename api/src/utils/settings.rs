// Libs
use config::{Config, ConfigError, Environment};
use serde::Deserialize;

// Schema
#[derive(Debug, Deserialize)]
pub struct Settings {
    // log_level: String,
}

// Functions
pub fn get_settings() -> Result<Settings, ConfigError> {
    Config::builder()
        .add_source(Environment::default())
        .build()?
        .try_deserialize()
}
