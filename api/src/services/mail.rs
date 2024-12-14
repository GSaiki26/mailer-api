use std::sync::Arc;

use axum::{http::StatusCode, Json};
use models::{attachment, mail};
use sea_orm::{prelude::Uuid, DatabaseConnection, EntityTrait, ModelTrait};
use tokio::sync::RwLock;
use tracing::{error, info};

use crate::schemas::{APIResponse, AttachmentIn, MailIn};

#[derive(Clone)]
pub struct MailService {
    pub db: Arc<RwLock<DatabaseConnection>>,
}

impl MailService {
    pub fn new(db: Arc<RwLock<DatabaseConnection>>) -> Self {
        Self { db }
    }

    pub async fn insert_mail(
        &self,
        mail: MailIn,
    ) -> Result<mail::Model, (StatusCode, Json<APIResponse>)> {
        info!("Inserting mail into database...");
        let db = self.db.write().await;

        let mail_actmodel: mail::ActiveModel = mail.clone().into();
        match mail::Entity::insert(mail_actmodel)
            .exec_with_returning(&*db)
            .await
        {
            Ok(created_mail) => {
                info!("Mail inserted successfully.");
                Ok(created_mail)
            }
            Err(err) => {
                error!(error = %err, "Failed to insert mail into database.");
                Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(APIResponse::error_with_message(String::from(
                        "Failed to save the mail.",
                    ))),
                ))
            }
        }
    }

    pub async fn find_mail(
        &self,
        mail_id: Uuid,
    ) -> Result<Option<mail::Model>, (StatusCode, Json<APIResponse>)> {
        let span = tracing::info_span!("", mail_id = %mail_id);
        let _guard = span.enter();

        info!("Finding mail in database...");
        let db = self.db.read().await;

        match mail::Entity::find_by_id(mail_id).one(&*db).await {
            Ok(mail) => {
                info!("Mail successfully found.");
                Ok(mail)
            }
            Err(err) => {
                error!(error = %err, "Failed to find mail in database.");
                Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(APIResponse::error_with_message(String::from(
                        "Failed to find the mail.",
                    ))),
                ))
            }
        }
    }

    pub async fn insert_attachments(
        &self,
        mail_id: Uuid,
        attachments: &[AttachmentIn],
    ) -> Result<(), (StatusCode, Json<APIResponse>)> {
        info!("Inserting attachments into database...");
        let db = self.db.write().await;

        let attachs_models: Vec<attachment::ActiveModel> = attachments
            .iter()
            .map(|att| att.to_model(mail_id))
            .collect();

        if attachs_models.is_empty() {
            info!("No attachments to insert.");
            return Ok(());
        }

        match attachment::Entity::insert_many(attachs_models)
            .exec_without_returning(&*db)
            .await
        {
            Ok(_) => {
                info!("Attachments inserted successfully.");
                Ok(())
            }
            Err(err) => {
                error!(error = %err, "Failed to insert attachments into database.");
                Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(APIResponse::error_with_message(String::from(
                        "Failed to save the attachments.",
                    ))),
                ))
            }
        }
    }

    pub async fn find_attachments(
        &self,
        mail: &mail::Model,
    ) -> Result<Vec<attachment::Model>, (StatusCode, Json<APIResponse>)> {
        info!("Fetching attachments from database...");
        let db = self.db.read().await;

        match mail.find_related(attachment::Entity).all(&*db).await {
            Ok(attachments) => {
                info!("Attachments fetched successfully.");
                Ok(attachments)
            }
            Err(err) => {
                error!(error = %err, "Failed to fetch attachments from database.");
                Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(APIResponse::error_with_message(String::from(
                        "Failed to fetch the attachments.",
                    ))),
                ))
            }
        }
    }
}
