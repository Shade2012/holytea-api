use std::sync::Arc;

use axum::{
    middleware, routing::{delete, get, post}, Router
};
use sqlx::postgres::PgPool;

use super::health_checker_handler;



pub struct AppState {
    pub db: PgPool,
 }

pub fn create_router(state: Arc<AppState>) -> Router {
    Router::new()
    .route("/api/healthcheck", get(health_checker_handler))
    // .route("/api/register", method_router)
    .with_state(state)
}