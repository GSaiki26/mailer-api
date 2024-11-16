use lettre::{
    transport::smtp, transport::smtp::authentication::Credentials, AsyncSmtpTransport,
    AsyncTransport, Message, Tokio1Executor,
};
use tracing::debug;

use crate::utils::SMTPSettings;

pub struct SMTPProvider {
    transport: AsyncSmtpTransport<Tokio1Executor>,
}

impl SMTPProvider {
    pub fn new(smtp_settings: &SMTPSettings) -> Self {
        debug!("Creating SMTPProvider...");
        let creds = Credentials::new(
            smtp_settings.username.clone(),
            smtp_settings.password.clone(),
        );

        Self {
            transport: lettre::AsyncSmtpTransport::<Tokio1Executor>::relay(&smtp_settings.host)
                .unwrap()
                .credentials(creds)
                .build(),
        }
    }

    pub async fn send_message(&self, message: Message) -> Result<(), smtp::Error> {
        self.transport.send(message).await?;
        Ok(())
    }
}
