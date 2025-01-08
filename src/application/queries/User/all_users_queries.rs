use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

use crate::{api::router::AppState, domain::models::users::{user_to_response, User, UserResponse}};

pub async fn all_users_queries(
    State(data): State<Arc<AppState>>,
)-> Result<impl IntoResponse,(StatusCode,Json<serde_json::Value>)>{
    let users = sqlx::query_as!(User,"SELECT * FROM users")
        .fetch_all(&data.db)
        .await
        .map_err(|_|{
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "status": "error",
                    "message": "An error occurred while fetching users",
                })),
            )
        })?;
        let users_response = users
        .iter()
        .map(|user| user_to_response(&user))
        .collect::<Vec<UserResponse>>();
    Ok((
        StatusCode::OK,
        Json(json!({
            "status": "success",
            "data": users_response,
        })),
    ))
}