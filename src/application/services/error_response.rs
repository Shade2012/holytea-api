use axum::{http::StatusCode, Json};
use serde_json::json;

pub fn error_response(message: &str) -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::BAD_REQUEST,
        Json(json!({
            "status": "error",
            "message": message,
        })),
    )
}