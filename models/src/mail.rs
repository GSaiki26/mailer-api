use lettre::{
    message::{header::ContentType, Attachment, MultiPart},
    Message,
};
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidateEmail, ValidationError};

use crate::{attachment, errors::ModelError};

fn are_valid_emails(emails: &Vec<String>) -> Result<(), ValidationError> {
    for email in emails {
        if !ValidateEmail::validate_email(email) {
            return Err(ValidationError::new("invalid email"));
        }
    }
    Ok(())
}

#[derive(Clone, Debug, DeriveEntityModel, Deserialize, Serialize, Validate)]
#[sea_orm(table_name = "mail")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,

    #[validate(custom(function = "are_valid_emails"))]
    pub to: Vec<String>,

    #[validate(custom(function = "are_valid_emails"))]
    pub cc: Vec<String>,

    #[validate(custom(function = "are_valid_emails"))]
    pub bcc: Vec<String>,

    pub subject: String,
    pub body: String,

    pub scheduled_at: DateTimeUtc,
    pub was_sent: bool,

    #[serde(default)]
    pub created_at: DateTimeUtc,

    #[serde(default)]
    pub updated_at: DateTimeUtc,
}

#[derive(Debug, DeriveRelation, EnumIter)]
pub enum Relation {
    #[sea_orm(has_many = "attachment::Entity")]
    Attachment,
}

impl ActiveModelBehavior for ActiveModel {}

impl Related<attachment::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Attachment.def()
    }
}

impl Model {
    pub async fn to_message(
        &self,
        db: &DatabaseConnection,
        sender: &str,
    ) -> Result<Message, ModelError> {
        let mut mail = Message::builder().from(sender.parse().unwrap());
        for to in self.to.clone() {
            mail = mail.to(to.parse().unwrap());
        }
        for cc in self.cc.clone() {
            mail = mail.cc(cc.parse().unwrap());
        }
        for bcc in self.bcc.clone() {
            mail = mail.bcc(bcc.parse().unwrap());
        }

        let attachs: Vec<attachment::Model> = self.find_related(attachment::Entity).all(db).await?;
        let mut multipart = MultiPart::related().build();
        for attach in attachs {
            let content_type = ContentType::parse(&attach.content_type).unwrap();
            let lettre_attach = Attachment::new(attach.filename).body(attach.content, content_type);
            multipart = multipart.singlepart(lettre_attach);
        }

        Ok(mail.multipart(multipart)?)
    }
}
