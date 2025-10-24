use axum::{
    Json,
    http::{StatusCode},
    response::IntoResponse,
};
use serde_json::json;

#[derive(Debug)]
pub enum AppError {
    TodoNotFound,
    InvalidInput(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match self {
            AppError::TodoNotFound => (StatusCode::NOT_FOUND, "Todo not found".to_string()),
            AppError::InvalidInput(msg) => (StatusCode::BAD_REQUEST, msg.to_string()),
        };

        let body = Json(json!({
            "error": message
        }));

        (status, body).into_response()
    }
}
