use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde_json::json;
use sqlx::query_as;

use crate::{api::router::AppState, domain::models::product::{product_to_response, Product, ProductResponse}};

pub async fn all_products_query(
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse,(StatusCode,Json<serde_json::Value>)>{
    let products = query_as!(
        Product,
        r#"SELECT * FROM products"#
    ).fetch_all( &data.db)
    .await.map_err(|_|{
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "status": "error",
                "message": "An error occurred while fetching products",
            })),
        )
    })?;
    let product_response:Vec<ProductResponse> = products.iter().map(|product|{
       product_to_response(product)
    }).collect();
    Ok((
        StatusCode::OK,
        Json(json!({
            "status": "success",
            "message": "All products fetched successfully",
            "data": product_response,
        })),
    ))

}