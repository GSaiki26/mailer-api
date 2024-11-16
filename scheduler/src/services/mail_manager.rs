use std::process::exit;

use chrono::Utc;
use models::mail;
use sea_orm::{ActiveModelTrait, ActiveValue, DatabaseConnection, DbErr};
use tracing::{error, info};

use crate::utils::MailSettings;

pub struct MailManagerService<'a> {
    db: &'a DatabaseConnection,
    pub mail_settings: &'a MailSettings,
}

impl<'a> MailManagerService<'a> {
    pub fn new(db: &'a DatabaseConnection, mail_settings: &'a MailSettings) -> Self {
        Self { db, mail_settings }
    }

    pub async fn add_schedule_backoff(&self, mail: mail::Model) {
        let backoff = std::time::Duration::from_secs(self.mail_settings.schedule_backoff_secs);
        let new_scheduled_at = mail.scheduled_at + backoff;

        info!(
            original_schedule = %mail.scheduled_at,
            new_schedule = %new_scheduled_at,
            "Adding backoff to mail...");
        let model_with_backoff = mail::ActiveModel {
            id: ActiveValue::Set(mail.id),
            scheduled_at: ActiveValue::Set(new_scheduled_at),
            updated_at: ActiveValue::Set(Utc::now()),
            ..Default::default()
        };

        info!("Updating mail with backoff...");
        if let Err(err) = model_with_backoff.update(self.db).await {
            error!(err = %err, "Failed to add backoff to mail.");
            exit(1);
        }

        info!("Backoff added to mail.");
    }

    pub async fn set_mail_as_sent(&self, mail: mail::Model) -> Result<(), DbErr> {
        info!("Setting mail as sent...");

        let new_mail_model = mail::ActiveModel {
            id: ActiveValue::Set(mail.id),
            was_sent: ActiveValue::Set(true),
            updated_at: ActiveValue::Set(Utc::now()),
            ..Default::default()
        };

        info!("Updating mail as sent...");
        new_mail_model.update(self.db).await?;

        info!("Mail set as sent.");
        Ok(())
    }
}
