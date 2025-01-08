use std::sync::Arc;

use axum::{
    middleware, routing::{delete, get, post}, Router
};
use sqlx::postgres::PgPool;

// use crate::application::commands::create_user_command::User::{self, create_user_command};

use crate::application::commands::User::create_user_command::create_user_command;

use super::health_checker_handler;



pub struct AppState {
    pub db: PgPool,
 }

pub fn create_router(state: Arc<AppState>) -> Router {
    Router::new()
    .route("/api/healthcheck", get(health_checker_handler))
    .route("/api/register", post(create_user_command))
    .route("/api/login", post())
    .with_state(state)
}