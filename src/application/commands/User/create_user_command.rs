use std::sync::Arc;
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use bcrypt::hash;
use serde_json::json;
use sqlx::query_as;
use crate::{api::router::AppState, domain::{models::users::{user_to_response, User}, schema::CreateUserSchema}};

pub async fn create_user_command(
    State(data): State<Arc<AppState>>,
    Json(payload): Json<CreateUserSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    // Hash the password
    let hash_password = hash(&payload.password, 8).map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "status": "error",
                "message": "Failed to hash password"
            })),
        )
    })?;

    // Insert the user into the database
    let user_result = query_as!(
        User,
        r#"
        INSERT INTO users (username, email, password) 
        VALUES ($1, $2, $3) 
        RETURNING id, username, email, password, created_at
        "#,
        payload.username,
        payload.email,
        hash_password
    )
    .fetch_one(&data.db)
    .await;

    // Handle the result of the user creation
    match user_result {
        Ok(user) => Ok((
            StatusCode::CREATED,
            Json(json!({
                "status": "success",
                "data": user_to_response(&user),
            })),
        )),
        Err(err) => {
            if err.to_string().contains("Duplicate") {
                Err((
                    StatusCode::CONFLICT,
                    Json(json!({
                        "status": "error",
                        "message": "Email is already registered"
                    })),
                ))
            } else {
                Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({
                        "status": "error",
                        "message": "Failed to create user"
                    })),
                ))
            }
        }
    }
}
