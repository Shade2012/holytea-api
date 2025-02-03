use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Extension, Json};
use jsonwebtoken::TokenData;
use serde_json::json;

use crate::{api::router::AppState, application::{middleware::auth::{decode_jwt, Claims}, services::error_response::error_response}, domain::{models::{payment::{payment_to_response, Payment}, product::{product_to_response, Product}}, schema::CreatePaymentSchema}};

pub async fn create_payment_command(
    State(data): State<Arc<AppState>>,
    Json(payload):Json<CreatePaymentSchema>
) -> Result<impl IntoResponse,(StatusCode,Json<serde_json::Value>)> {
    let product_price = sqlx::query!(
        r#"
        SELECT product_price FROM products WHERE id = $1
        "#,
        payload.id_product
    ).fetch_one(&data.db)
    .await
    .map_err(|_| {
        return error_response("Failed to get product price")
    })?
    .product_price;

    let total_price  = product_price * (payload.product_amount as i32);
    let payment = sqlx::query_as!(
        Payment,
        r#"INSERT INTO payment (id_payment_history, id_product, payment_product_price, product_amount)
        VALUES ($1, $2, $3, $4)
        RETURNING id, id_payment_history, id_product, payment_product_price, product_amount
     "#,
     1,
     payload.id_product,
     total_price,
     payload.product_amount
    ) .fetch_one(&data.db)
    .await
    .map_err(|_| {
        return error_response("Failed to create payment")
    })?;
    let product = sqlx::query_as!(
        Product,
        r#"SELECT * FROM products WHERE id = $1
     "#,
     payment.id_product
    ) .fetch_one(&data.db)
    .await
    .map_err(|_| {
        return error_response("Failed to create payment")
    })?;
    Ok((
        StatusCode::CREATED,
        Json(json!({
            "status": "success",
            "message": "Product created successfully",
            "data": payment_to_response(&payment,product_to_response(&product)),
        })),
    ))
}
