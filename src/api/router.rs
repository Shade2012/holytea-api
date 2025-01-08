use std::sync::Arc;

use axum::{
    middleware, routing::{delete, get, post}, Router
};
use sqlx::postgres::PgPool;

// use crate::application::commands::create_user_command::User::{self, create_user_command};

use crate::application::{commands::User::create_user_command::create_user_command, middleware::auth, queries::User::{all_users_queries::all_users_queries, login_user_queries::login_user_queries}};

use super::health_checker_handler;



pub struct AppState {
    pub db: PgPool,
 }

pub fn create_router(state: Arc<AppState>) -> Router {
    Router::new()
    .route("/api/healthcheck", get(health_checker_handler))
    .route("/api/register", post(create_user_command))
    .route("/api/login", post(login_user_queries))
    .nest("/api", Router::new()
        .route("/all_users", get(all_users_queries))
        .layer(middleware::from_fn(auth::middleware))
    )
    .with_state(state)
}