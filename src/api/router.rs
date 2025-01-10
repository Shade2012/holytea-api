
use std::sync::Arc;
use axum::{
    extract::DefaultBodyLimit, middleware, routing::{delete, get, get_service, post}, Router, ServiceExt
};
use sqlx::postgres::PgPool;
use tower_http::services::ServeDir;

// use crate::application::commands::create_user_command::User::{self, create_user_command};

use crate::application::{commands::{Product::{create_product_command::create_product_command, delete_product_command::delete_product_command, update_products_command::update_products_command}, User::create_user_command::create_user_command}, middleware::auth, queries::{Product::{all_products_query::all_products_query, detail_product_query::detail_product_query}, User::{all_users_queries::all_users_queries, login_user_queries::login_user_queries}}};

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

    //User routes
    .nest("/users", Router::new()
        .route("/all_users", get(all_users_queries))
    )
        
    //Product routes
        .nest(    
        "/product", Router::new()
        .route("/all-products", get(all_products_query))
        .route("/detail-product/:id", get(detail_product_query))
        .route("/add-product", post(create_product_command))
        .route("/update-product/:id", post(update_products_command))
        .route("/delete-product/:id", delete(delete_product_command))
    )  

    //Auth Middleawre
        .layer(middleware::from_fn(auth::middleware))
    )
    .nest_service("/public", get_service(ServeDir::new("public")).handle_error(|error| async move{
        (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            format!("Unhandled error: {}", error),
        )
    }))
    .layer(DefaultBodyLimit::disable())
    .with_state(state)
}
