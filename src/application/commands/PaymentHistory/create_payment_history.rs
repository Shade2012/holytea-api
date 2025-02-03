use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Extension, Json};

use crate::{api::router::AppState, application::services::error_response::{self, error_response}, domain::{models::{payment::Payment, payment_history::{PaymentHistory, StatusPayment}}, schema::{CreatePaymentHistorySchema, CreatePaymentSchema}}};

pub async fn create_payment_history_command (
    State(data): State<Arc<AppState>>,
    Extension(user_id): Extension<i32>,
    Json(payload): Json<CreatePaymentHistorySchema>
) -> Result<impl IntoResponse,(StatusCode,Json<serde_json::Value>)>{
    // Start a transaction to ensure atomicity
    let payment_history = sqlx::query_as!(
        PaymentHistory,
        r#"
        INSERT INTO payment_history (user_id, user_amount_money, total_price, status_payment)
        VALUES ($1, $2, $3, $4)
        RETURNING id, user_id, user_amount_money, total_price, status_payment AS "status_payment!: StatusPayment", created_at, updated_at
        "#,
        user_id,
        payload.user_amount_money,
        0,
        StatusPayment::Pending as StatusPayment
    )
    .fetch_one(&data.db)
    .await
    .map_err(|e| error_response(format!("failed to insert payment history {e}").as_str()))?;
    
    let result = handle_create_payment_history(&data, payment_history.id.unwrap_or_default(),payload.list_payment).await;
    if result.is_ok(){
        let total_price_payment = sqlx::query!(
            r#"
            SELECT SUM(payment_product_price)
            FROM payment
            WHERE id_payment_history = $1
            "#,
            payment_history.id.unwrap_or_default()
        )
        .fetch_one(&data.db)
        .await
        .map_err(|_|error_response("failed to update total price"))?.sum;

        sqlx::query(
            r#"
        UPDATE payment_history
        SET total_price = $1
        WHERE id = $2
        "#,
        )
        .bind(total_price_payment.unwrap_or_default())
        .bind(payment_history.id.unwrap_or_default())
        .execute(&data.db).await.map_err(|e|error_response(e.to_string().as_str()))?;
        return Ok((StatusCode::CREATED, Json(serde_json::json!({
            "status": "success",
            "message": "Payment created successfully"
        }))));
    }else{
        return Err(
            result.err().unwrap()
        );
    }
    
   
}

async fn handle_create_payment_history(
    data: &Arc<AppState>,
    payment_history_id:i32,
    payload: Vec<CreatePaymentSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
     for payment_detail in payload {
        let product_price = sqlx::query!(
            r#"
            SELECT product_price FROM products WHERE id = $1
            "#,
            payment_detail.id_product
        ).fetch_one(&data.db)
        .await
        .map_err(|_| {
            return error_response("Failed to get product price")
        })?
        .product_price;
    
        let total_price  = product_price * (payment_detail.product_amount as i32);
        sqlx::query!(
            r#"INSERT INTO payment (id_payment_history, id_product, payment_product_price, product_amount)
            VALUES ($1, $2, $3, $4)
         "#,
         payment_history_id,
         payment_detail.id_product,
         total_price,
         payment_detail.product_amount
        ) .execute(&data.db)
        .await
        .map_err(|_| {
            return error_response("Failed to create payment")
        })?;
    };
    // Your logic here
    Ok((
        StatusCode::CREATED,
        Json(serde_json::json!({
            "status": "success",
            "message": "Payment created successfully"
        }))
    ))
}