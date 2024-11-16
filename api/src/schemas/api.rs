use serde::Serialize;

#[derive(Serialize)]
enum APIStatus {
    #[serde(rename = "success")]
    Success,

    #[serde(rename = "error")]
    Error,
}

#[derive(Serialize)]
pub struct APIResponse {
    status: APIStatus,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

impl APIResponse {
    pub fn error_with_message(message: String) -> Self {
        Self {
            status: APIStatus::Error,
            message: Some(message),
            data: None,
        }
    }

    pub fn success_with_data<T: Serialize>(data: T) -> Self {
        Self {
            status: APIStatus::Success,
            message: None,
            data: Some(serde_json::to_value(data).unwrap()),
        }
    }
}
