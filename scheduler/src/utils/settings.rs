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

#[derive(Debug, Deserialize, Validate)]
pub struct MailSettings {
    pub cooldown_secs: u64,
    pub schedule_backoff_secs: u64,
}

impl Settings for MailSettings {
    fn from_env() -> Result<MailSettings, ConfigError> {
        Config::builder()
            .add_source(Environment::default().prefix("MAIL"))
            .build()?
            .try_deserialize()
    }
}

#[derive(Debug, Deserialize, Validate)]
pub struct SMTPSettings {
    #[validate(url)]
    pub host: String,
    pub username: String,
    pub password: String,

    #[validate(email)]
    pub sender: String,
}

impl Settings for SMTPSettings {
    fn from_env() -> Result<SMTPSettings, ConfigError> {
        Config::builder()
            .add_source(Environment::default().prefix("SMTP"))
            .build()?
            .try_deserialize()
    }
}
