use models::mail;
use sea_orm::DatabaseConnection;
use tracing::info;

use crate::{errors::SendMailError, providers::SMTPProvider, utils::SMTPSettings};

pub struct SMTPService {
    prov: SMTPProvider,
    smtp_settings: SMTPSettings,
}

impl SMTPService {
    pub fn new(smtp_settings: SMTPSettings, prov: SMTPProvider) -> Self {
        Self {
            prov,
            smtp_settings,
        }
    }

    pub async fn send_mail(
        &self,
        db: &DatabaseConnection,
        mail: &mail::Model,
    ) -> Result<(), SendMailError> {
        info!("Preparing mail...");
        let message = mail.to_message(db, &self.smtp_settings.sender).await?;

        info!("Sending mail...");
        self.prov.send_message(message).await?;

        info!("Mail successfully sent.");
        Ok(())
    }
}
