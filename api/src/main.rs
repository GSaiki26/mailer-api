use axum::{
    routing::{get, post},
    Router,
};
use controllers::{get_mail, post_mail};
use schemas::APIState;
use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr};
use services::MailService;
use std::{sync::Arc, time::Duration};
use tokio::{net::TcpListener, sync::RwLock};
use tracing::info;
use utils::{setup_logger, DatabaseSettings, Settings};

mod controllers;
mod errors;
mod schemas;
mod services;
mod utils;

pub async fn get_db() -> Result<DatabaseConnection, DbErr> {
    let db_settings = DatabaseSettings::from_env().expect("Failed to load database settings");
    let timeout_duration = Duration::from_secs(db_settings.timeout_secs);

    info!("Connecting to database...");
    let db = Database::connect(
        ConnectOptions::new(db_settings.dsn)
            .connect_timeout(timeout_duration)
            .to_owned(),
    )
    .await;

    info!("Connected to database.");
    db
}

#[tokio::main]
async fn main() {
    setup_logger();

    let db = RwLock::const_new(get_db().await.expect("Failed to connect to database"));
    let mail_svc = MailService::new(Arc::new(db));

    let state = APIState {
        mail_svc: Arc::new(mail_svc),
    };
    let router = Router::new()
        .route("/mail", post(post_mail))
        .route("/mail/:mail_id", get(get_mail));

    let app = Router::new().nest("/api/v1", router).with_state(state);
    let addr = "0.0.0.0:3000";

    info!(addr = addr, "Starting server...");
    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
