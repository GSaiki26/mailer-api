use lettre::{
    transport::smtp, transport::smtp::authentication::Credentials, AsyncSmtpTransport,
    Tokio1Executor,
};
use sea_orm::{Database, DatabaseConnection, DbErr};
pub use settings::*;
use tracing_subscriber::{self, EnvFilter};

mod settings;

pub fn setup_logger() {
    let env_filter = EnvFilter::from_env("LOG_LEVEL");
    tracing_subscriber::fmt().with_env_filter(env_filter).init();
}

pub async fn get_db() -> Result<DatabaseConnection, DbErr> {
    let db_settings = DatabaseSettings::from_env().expect("Failed to load database settings");

    Database::connect(db_settings.dsn).await
}

pub async fn get_transport() -> Result<AsyncSmtpTransport<Tokio1Executor>, smtp::Error> {
    let smtp_settings = SMTPSettings::from_env().expect("Failed to load mail settings");
    let creds = Credentials::new(smtp_settings.username, smtp_settings.password);

    Ok(
        lettre::AsyncSmtpTransport::<Tokio1Executor>::relay(&smtp_settings.host)?
            .credentials(creds)
            .build(),
    )
}
