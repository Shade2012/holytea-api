use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use bcrypt::verify;
use serde_json::json;
use sqlx::query_as;

use crate::{api::router::AppState, domain::schema::LoginUserSchema};

pub async fn login_user_queries(
State(data):State<Arc<AppState>>,
Json(body):Json<LoginUserSchema>
) -> Result<impl IntoResponse,(StatusCode,Json<serde_json::Value>)>
{
    let user = query_as!(User,r#"SELECT * FROM users WHERE name = ?"#,&body.email)
        .fetch_one(&data.db)
        .await
        .map_err(|e|{
            (
                StatusCode::NOT_FOUND,
                Json(json!({
                   "status" : "Error",
                   "message": "Email not found",
                })),
            )
        })?;
        let password_match : bool = verify(&body.password, &user.password).map_err(|_|{
            (
                StatusCode::NOT_FOUND,
                Json(json!({
                   "status" : "Error",
                   "message": "An error occurred during password verification",
                })),
            )
        })?;
        if !password_match {
            return Err((
                StatusCode::UNAUTHORIZED,
                Json(json!({
                    "status": "error",
                    "message": "Invalid username or password",
                })),
            ));
        }
        let token = 
    Ok(())
}