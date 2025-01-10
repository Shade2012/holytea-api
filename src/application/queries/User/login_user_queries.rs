use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use bcrypt::verify;
use serde_json::json;
use sqlx::query_as;

use crate::{api::router::AppState, application::middleware::auth::encode_jwt, domain::{models::users::User, schema::LoginUserSchema}};

pub async fn login_user_queries(
State(data):State<Arc<AppState>>,
Json(body):Json<LoginUserSchema>
) -> Result<impl IntoResponse,(StatusCode,Json<serde_json::Value>)>
{
    let user = query_as!(User,r#"SELECT * FROM users WHERE email = $1"#,&body.email)
        .fetch_one(&data.db)
        .await
        .map_err(|_|{
            (
                StatusCode::NOT_FOUND,
                Json(json!({
                   "status" : "Error",
                   "message": "Email not found",
                })),
            )
        })?;
        let password_match : bool = verify(&body.password, user.password.unwrap().as_str()).map_err(|_|{
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
        let token = encode_jwt(user.id.unwrap()).await;
        match token {
            Ok(token) => {
                Ok((
                    StatusCode::OK,
                    Json(json!({
                        "status": "success",
                        "message": "Login successful",
                        "token": token,
                    })),
                ))
            }
            Err(_) => {
                Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({
                        "status": "error",
                        "message": "An error occurred during token generation",
                    })),
                ))
            }
            
        }
}