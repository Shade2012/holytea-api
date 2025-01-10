use std::sync::Arc;

use axum::{extract::{Path, State}, http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

use crate::{api::router::AppState, domain::models::product::{product_to_response, Product}};

pub async fn detail_product_query(
    State(data): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse,(StatusCode, Json<serde_json::Value>)>{
    let product = sqlx::query_as!(
        Product,
        r#"
        SELECT * FROM products WHERE id = $1
        "#
        ,id
    )
    .fetch_one(&data.db)
    .await
    .map_err(|e|{
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "status": "error",
                "error": e.to_string(),
                "message": "An error occurred while fetching product",
            })),
        )
    })?;
    Ok(
        (
            StatusCode::OK,
            Json(json!({
                "status": "success",
                "message": "Product fetched successfully",
                "data": product_to_response(&product),
            })),
        )
    )
}