use base64::prelude::*;
use chrono::Utc;
use mime_guess::mime::OCTET_STREAM;
use models::attachment;
use once_cell::sync::Lazy;
use regex::Regex;
use sea_orm::{prelude::Uuid, ActiveValue};
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

static FILENAME_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[\w][\w.-]*\.[\w]+$").unwrap());

fn is_base64(s: &str) -> Result<(), ValidationError> {
    match BASE64_STANDARD.decode(s.as_bytes()) {
        Ok(_) => Ok(()),
        Err(_) => Err(ValidationError::new("Invalid base64 encoding.")),
    }
}

#[derive(Clone, Deserialize, Validate)]
pub struct AttachmentIn {
    #[validate(regex(path = *FILENAME_REGEX))]
    pub filename: String,

    #[validate(custom(function = "is_base64"))]
    pub content: String,
}

impl AttachmentIn {
    pub fn to_model(&self, mail_id: Uuid) -> attachment::ActiveModel {
        let created_at = Utc::now();

        let default_mime = OCTET_STREAM.to_string().parse().unwrap();
        let content_tipe = mime_guess::from_path(&self.filename).first_or(default_mime);

        let content = BASE64_STANDARD.decode(self.content.as_bytes()).unwrap();

        attachment::ActiveModel {
            id: ActiveValue::Set(Uuid::now_v7()),
            mail_id: ActiveValue::Set(mail_id),
            filename: ActiveValue::Set(self.filename.clone()),
            content: ActiveValue::Set(content),
            content_type: ActiveValue::Set(content_tipe.to_string()),
            created_at: ActiveValue::Set(created_at),
            updated_at: ActiveValue::Set(created_at),
        }
    }
}

#[derive(Clone, Serialize, Validate)]
pub struct AttachmentOut {
    pub id: Uuid,
    pub mail_id: Uuid,
    pub filename: String,
    pub content: String,
    pub content_type: String,
    pub created_at: String,
    pub updated_at: String,
}

impl From<attachment::Model> for AttachmentOut {
    fn from(att: attachment::Model) -> Self {
        Self {
            id: att.id,
            mail_id: att.mail_id,
            filename: att.filename,
            content: BASE64_STANDARD.encode(&att.content),
            content_type: att.content_type,
            created_at: att.created_at.to_string(),
            updated_at: att.updated_at.to_string(),
        }
    }
}
