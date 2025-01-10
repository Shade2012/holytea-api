use std::sync::Arc;

use axum::{extract::{Path, State}, http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

use crate::api::router::AppState;

pub async fn delete_product_command (
    State(data):State<Arc<AppState>>,
    Path(id):Path<i32>,
) -> Result<impl IntoResponse,(StatusCode,Json<serde_json::Value>)>{
    let query = sqlx::query!(
        r#"
        DELETE FROM products WHERE id = $1
        "#,
        id
    ).execute(&data.db)
    .await.map_err(|_|{
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "status": "error",
                "message": "An error occurred while deleting product",
            })),
        )
    })?;
    
    if query.rows_affected() == 0 {
        return Ok((
            StatusCode::NOT_FOUND,
            Json(json!({
                "status": "error",
                "message": "Product not found",
            })),
        ));
    }

    Ok((
        StatusCode::OK,
        Json(json!({
            "status": "success",
            "message": "Product deleted successfully",
        })),
    ))

}