use chrono::Utc;
use models::mail;
use sea_orm::{
    prelude::{DateTimeUtc, Uuid},
    ActiveValue,
};
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidateEmail, ValidationError};

use super::attachment::{AttachmentIn, AttachmentOut};

fn are_valid_emails(emails: &Vec<String>) -> Result<(), ValidationError> {
    for email in emails {
        if !email.validate_email() {
            return Err(ValidationError::new("Some provided email is invalid."));
        }
    }

    Ok(())
}

fn are_valid_target_emails(emails: &Vec<String>) -> Result<(), ValidationError> {
    if emails.is_empty() {
        return Err(ValidationError::new(
            "At least one target email is required.",
        ));
    }

    are_valid_emails(emails)
}

fn is_valid_scheduled_at(scheduled_at: &DateTimeUtc) -> Result<(), ValidationError> {
    if scheduled_at <= &Utc::now() {
        return Err(ValidationError::new("scheduled_at must be in the future."));
    }

    Ok(())
}

#[derive(Clone, Deserialize, Validate)]
pub struct MailIn {
    #[validate(custom(function = "are_valid_target_emails"))]
    pub to: Vec<String>,

    #[validate(custom(function = "are_valid_emails"))]
    #[serde(default)]
    pub cc: Vec<String>,

    #[validate(custom(function = "are_valid_emails"))]
    #[serde(default)]
    pub bcc: Vec<String>,

    pub subject: String,
    pub body: String,

    #[validate(custom(function = "is_valid_scheduled_at"))]
    #[serde(default)]
    pub scheduled_at: Option<DateTimeUtc>,

    #[validate(nested)]
    #[serde(default)]
    pub attachments: Vec<AttachmentIn>,
}

impl From<MailIn> for mail::ActiveModel {
    fn from(mail: MailIn) -> Self {
        let created_at = Utc::now();
        let scheduled_at = mail.scheduled_at.unwrap_or_else(|| created_at);

        mail::ActiveModel {
            id: ActiveValue::Set(Uuid::now_v7()),
            to: ActiveValue::Set(mail.to),
            cc: ActiveValue::Set(mail.cc),
            bcc: ActiveValue::Set(mail.bcc),
            subject: ActiveValue::Set(mail.subject),
            body: ActiveValue::Set(mail.body),
            scheduled_at: ActiveValue::Set(scheduled_at),
            was_sent: ActiveValue::Set(false),
            created_at: ActiveValue::Set(created_at),
            updated_at: ActiveValue::Set(created_at),
        }
    }
}

#[derive(Clone, Serialize, Validate)]
pub struct MailOut {
    pub id: Uuid,
    pub to: Vec<String>,
    pub cc: Vec<String>,
    pub bcc: Vec<String>,
    pub subject: String,
    pub body: String,
    pub scheduled_at: DateTimeUtc,
    pub was_sent: bool,
    pub attachments: Vec<AttachmentOut>,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
}
