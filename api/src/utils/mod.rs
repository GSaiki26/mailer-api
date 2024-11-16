use models::mail;
pub use settings::*;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::{self, EnvFilter};

use crate::schemas::{AttachmentOut, MailOut};

mod settings;

pub fn setup_logger() {
    let other_settings = OtherSettings::from_env().expect("Failed to load other settings");
    let filter = EnvFilter::new("")
        .add_directive(
            format!("api={}", other_settings.log_level.to_lowercase())
                .parse()
                .unwrap(),
        )
        .add_directive(LevelFilter::ERROR.into());

    tracing_subscriber::fmt().with_env_filter(filter).init();
}

pub fn mail_model_to_out(mail: mail::Model, attachments: Vec<AttachmentOut>) -> MailOut {
    MailOut {
        id: mail.id,
        to: mail.to,
        cc: mail.cc,
        bcc: mail.bcc,
        subject: mail.subject,
        body: mail.body,
        scheduled_at: mail.scheduled_at,
        attachments,
        created_at: mail.created_at.to_string(),
        updated_at: mail.updated_at.to_string(),
    }
}
