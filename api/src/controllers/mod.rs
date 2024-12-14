use axum::{http::StatusCode, Json};
pub use mail::{get_mail, post_mail};
use tracing::info;
use validator::Validate;

use crate::schemas::APIResponse;

mod mail;

fn validate_body<T: Validate>(body: &T) -> Result<(), (StatusCode, Json<APIResponse>)> {
    info!("Validating request body...");

    if let Err(err) = body.validate() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(APIResponse::error_with_message(err.to_string())),
        ));
    }

    Ok(())
}
