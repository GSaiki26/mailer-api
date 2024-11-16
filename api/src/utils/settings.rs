use config::{Config, ConfigError, Environment};
use serde::Deserialize;
use validator::Validate;

pub trait Settings {
    fn from_env() -> Result<Self, ConfigError>
    where
        Self: Sized;
}

#[derive(Debug, Deserialize, Validate)]
pub struct OtherSettings {
    pub log_level: String,
}

impl Settings for OtherSettings {
    fn from_env() -> Result<OtherSettings, ConfigError> {
        Config::builder()
            .add_source(Environment::default())
            .build()?
            .try_deserialize()
    }
}

#[derive(Debug, Deserialize, Validate)]
pub struct DatabaseSettings {
    #[validate(url)]
    pub dsn: String,

    pub timeout_secs: u64,
}

impl Settings for DatabaseSettings {
    fn from_env() -> Result<DatabaseSettings, ConfigError> {
        Config::builder()
            .add_source(Environment::default().prefix("DATABASE"))
            .build()?
            .try_deserialize()
    }
}
