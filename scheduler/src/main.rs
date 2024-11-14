use std::process::exit;

use lettre::AsyncTransport;
use models::{errors::ModelError, mail};
use schemas::State;
use sea_orm::{
    sea_query::SimpleExpr, ActiveModelTrait, ActiveValue, ColumnTrait, DbErr, EntityTrait,
    QueryFilter,
};
use tokio::time::sleep;
use tracing::{error, info, span, warn};
use utils::{get_db, get_transport, setup_logger, MailSettings, SMTPSettings, Settings};

mod schemas;
mod utils;

const ERROR_CREATE_MSG_LETTRE: &str = "Failed to create message due to bad creation. As the mail probably is invalid, it'll loop forever. Call the support.";
const ERROR_CREATE_MSG_DB: &str =
    "Failed to create message due to database error. Check the database status.";

#[tokio::main]
async fn main() {
    setup_logger();

    let state = State {
        db: &get_db().await.expect("Failed to connect to database"),
        mail_settings: &MailSettings::from_env().expect("Failed to load mail settings"),
        sender: SMTPSettings::from_env()
            .expect("Failed to load mail settings")
            .sender,
        transport: &get_transport().await.expect("Failed to create transport"),
    };

    let sleep_duration = std::time::Duration::from_secs(60 * state.mail_settings.cooldown_min);

    loop {
        let mails_to_send = get_mails_to_send(&state).await;
        if let Err(err) = mails_to_send {
            error!("Failed to fetch mails: {:?}", err);
            sleep(sleep_duration).await;
            continue;
        };
        send_mails(&state, mails_to_send.unwrap()).await;

        info!(sleep_min = state.mail_settings.cooldown_min, "Sleeping...");
        sleep(std::time::Duration::from_secs(
            state.mail_settings.cooldown_min,
        ))
        .await;
    }
}

fn get_mail_filter() -> SimpleExpr {
    let current_time = chrono::Utc::now();

    mail::Column::WasSent
        .eq(false)
        .and(mail::Column::ScheduledAt.lte(current_time))
}

async fn get_mails_to_send<'a>(state: &State<'a>) -> Result<Vec<mail::Model>, DbErr> {
    mail::Entity::find()
        .filter(get_mail_filter())
        .all(state.db)
        .await
}

async fn send_mails<'a>(state: &State<'a>, mails_to_send: Vec<mail::Model>) {
    let mut can_send = true;
    for mail in mails_to_send {
        let span = span!(
            tracing::Level::INFO,
            "send_mails",
            mail_id = String::from(mail.id)
        );
        let _enter = span.enter();

        if !can_send {
            info!("Send to mail is disabled.");
            if let Err(err) = add_schedule_backoff(state, mail).await {
                error!(err = format!("{:?}", err), "Failed to add backoff to mail.");
                exit(1);
            }
            continue;
        }

        let message = match mail.to_message(state.db, &state.sender).await {
            Ok(message) => message,
            Err(ModelError::LettreError(err)) => {
                warn!(err = format!("{:?}", err), ERROR_CREATE_MSG_LETTRE);
                continue;
            }
            Err(ModelError::DatabaseError(err)) => {
                error!(err = format!("{:?}", err), ERROR_CREATE_MSG_DB);
                continue;
            }
        };

        if let Err(err) = state.transport.send(message).await {
            error!(err = format!("{:?}", err), "Failed to send message.");
            can_send = false;
            continue;
        }

        if let Err(err) = set_mail_as_sent(state, mail).await {
            error!(err = format!("{:?}", err), "Failed to set mail as sent.");
            exit(1);
        }
    }
}

async fn add_schedule_backoff<'a>(state: &State<'a>, mail: mail::Model) -> Result<(), DbErr> {
    info!("Adding backoff to mail...");

    let backoff = std::time::Duration::from_secs(60 * state.mail_settings.schedule_backoff_min);
    let new_scheduled_at = mail.scheduled_at + backoff;

    let model_with_backoff = mail::ActiveModel {
        scheduled_at: ActiveValue::Set(new_scheduled_at),
        ..Default::default()
    };

    model_with_backoff.update(state.db).await?;

    info!("Backoff added to mail.");
    Ok(())
}

async fn set_mail_as_sent<'a>(state: &State<'a>, mail: mail::Model) -> Result<(), DbErr> {
    info!("Setting mail as sent...");

    let new_mail_model = mail::ActiveModel {
        id: ActiveValue::Set(mail.id),
        was_sent: ActiveValue::Set(true),
        ..Default::default()
    };

    new_mail_model.update(state.db).await?;

    info!("Mail set as sent.");
    Ok(())
}
