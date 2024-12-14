use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use tracing::info;
use uuid::Uuid;

use crate::{
    controllers::validate_body,
    schemas::{APIResponse, APIState, MailIn},
    utils::mail_model_to_out,
};

// #[axum::debug_handler]
pub async fn post_mail(
    State(state): State<APIState>,
    Json(mail): Json<MailIn>,
) -> Result<(StatusCode, Json<APIResponse>), (StatusCode, Json<APIResponse>)> {
    info!(method = "POST", route = "/api/v1/mail", "Received request.");
    validate_body(&mail)?;

    let created_mail = state.mail_svc.insert_mail(mail.clone()).await?;

    state
        .mail_svc
        .insert_attachments(created_mail.id, &mail.attachments)
        .await?;

    let attachments = state.mail_svc.find_attachments(&created_mail).await?;

    Ok((
        StatusCode::CREATED,
        Json(APIResponse::success_with_data(mail_model_to_out(
            created_mail,
            attachments.into_iter().map(|att| att.into()).collect(),
        ))),
    ))
}

pub async fn get_mail(
    State(state): State<APIState>,
    Path(mail_id): Path<Uuid>,
) -> Result<(StatusCode, Json<APIResponse>), (StatusCode, Json<APIResponse>)> {
    info!(method = "GET", route = "/api/v1/mail", "Received request.");

    let mail = state.mail_svc.find_mail(mail_id).await?;

    if mail.is_none() {
        info!("Returning result...");
        return Ok((
            StatusCode::NOT_FOUND,
            Json(APIResponse::error_with_message(
                "Mail not found.".to_string(),
            )),
        ));
    }

    let mail = mail.unwrap();
    let attachments = state.mail_svc.find_attachments(&mail).await?;

    info!("Returning result...");
    return Ok((
        StatusCode::CREATED,
        Json(APIResponse::success_with_data(mail_model_to_out(
            mail,
            attachments.into_iter().map(|att| att.into()).collect(),
        ))),
    ));
}
