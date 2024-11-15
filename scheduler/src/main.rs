use std::process::exit;
use std::time::Duration;

use crate::SendMailError::*;
use errors::SendMailError;
use models::errors::ModelError::*;
use providers::{get_mails_to_send, SMTPProvider};
use sea_orm::{ConnectOptions, DbErr};
use sea_orm::{Database, DatabaseConnection};
use services::{MailManagerService, SMTPService};
use tokio::time::sleep;
use tracing::{error, info, warn};
use utils::{setup_logger, DatabaseSettings, MailSettings, SMTPSettings, Settings};

mod errors;
mod providers;
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

    let mail_settings = MailSettings::from_env().expect("Failed to load mail settings");
    let sleep_duration = Duration::from_secs(mail_settings.cooldown_secs);

    let db = get_db().await.expect("Failed to connect to database");

    let smtp_settings = SMTPSettings::from_env().expect("Failed to load mail settings");
    let smtp_prov = SMTPProvider::new(&smtp_settings);
    let smtp_svc = SMTPService::new(smtp_settings, smtp_prov);

    let mail_mgr = MailManagerService::new(&db, &mail_settings);

    loop {
        let mails_to_send = get_mails_to_send(&db).await;
        if let Err(err) = mails_to_send {
            error!(err = format!("{:?}", err), "Failed to fetch mails.");
            exit(1);
        };

        let mut can_send = true;
        for mail in mails_to_send.unwrap() {
            if !can_send {
                info!("Send to mail is disabled.");
                mail_mgr.add_schedule_backoff(mail).await;
                continue;
            }

            match smtp_svc.send_mail(&db, &mail).await {
                Err(MessageCreationError(LettreError(err))) => {
                    warn!(
                        err = format!("{:?}", err),
                        "Couldn\'t create message. As the mail is invalid, it'll loop forever."
                    );
                }
                Err(MessageCreationError(DatabaseError(err))) => {
                    error!(
                        err = format!("{:?}", err),
                        "Couldn\'t create message due to database error. Check the database status."
                    );
                    exit(1);
                }
                Err(SendMailError::TransportError(err)) => {
                    warn!(
                        err = format!("{:?}", err),
                        "Couldn\'t send message due to transport error."
                    );
                    can_send = false;
                    mail_mgr.add_schedule_backoff(mail).await;
                }
                Ok(_) => {
                    if let Err(err) = mail_mgr.set_mail_as_sent(mail).await {
                        error!(
                            err = format!("{:?}", err),
                            "Couldn\'t set mail as sent. Check the database status."
                        );
                        exit(1);
                    }
                }
            }
        }

        info!(sleep_secs = mail_settings.cooldown_secs, "Sleeping...");
        sleep(sleep_duration).await;
    }
}
