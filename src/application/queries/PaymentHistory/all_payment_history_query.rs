use std::sync::Arc;
use axum::{extract::{Query, State}, http::StatusCode, response::IntoResponse, Extension, Json};
use crate::{
    api::router::AppState,
    application::services::error_response::error_response,
    domain::models::{payment::{payment_to_response, Payment, PaymentResponse}, payment_history::{PaymentHistory, PaymentHistoryResponse}, product::{product_to_response, Product}},
};

pub async fn all_payment_history_query(
    State(data): State<Arc<AppState>>,
    Query(params): Query<Vec<(String, String)>>,
    Extension(user_id): Extension<i32>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
  let mut query = String::from(
    r#"
    SELECT 
    id, user_id, user_amount_money, resi_number, total_price, 
    status_payment, created_at, updated_at
    FROM payment_history
    "#,
);

let mut query = format!(
    r#"
    SELECT 
    id, user_id, user_amount_money, resi_number, total_price, 
    status_payment, created_at, updated_at
    FROM payment_history
    WHERE user_id = {}
    "#,
    user_id
);

if !params.is_empty() {
    for (key, value) in params.iter() {
        if key == "resi_number" {
            query.push_str(&format!(" AND {} LIKE '%{}%'", key, value)); // Add % for LIKE
        } else {
            query.push_str(&format!(" AND {} = '{}'", key, value));
        }
    }
}
    let payment_query = sqlx::query_as(
        query.as_str()
    )
    .fetch_all(&data.db)
    .await
    .map_err(|e|
        error_response(format!("Failed to get payment {:?}",e).as_str())
    )?;
    let data = payment_history(payment_query,&data).await.map_err(|e|error_response(format!("Failed to get data payment {:?}",e).as_str()))?;
    Ok((
        StatusCode::OK,
        Json(serde_json::json!({
            "status": "success",
            "message": "Successfully get all payment history",
            "data": data
        })),
    ))
}

async fn payment_history(payment_history_list: Vec<PaymentHistory>,data: &Arc<AppState>) -> Result<Vec<PaymentHistoryResponse>, (StatusCode, Json<serde_json::Value>)>{
    let mut list_payment_response: Vec<PaymentHistoryResponse> = Vec::new();
    for payment_history  in payment_history_list {
        let payment_list = sqlx::query_as!(
            Payment,
            r#"
        SELECT *
        FROM payment
        WHERE id_payment_history = $1;
        "#,
        payment_history.id
        )
        .fetch_all(&data.db)
        .await
        .map_err(|e| error_response(format!("Failed to get payment history {e}").as_str()))?;
    let payment_response = payment(payment_list, data).await.unwrap();
    let response = PaymentHistoryResponse{
        id:payment_history.id.unwrap(),
        resi_number:payment_history.resi_number.unwrap(),
        user_id:payment_history.user_id,
        user_amount_money:payment_history.user_amount_money,
        total_price:payment_history.total_price,
        status_payment:payment_history.status_payment,
        product_payment: payment_response,
        updated_at:payment_history.updated_at.unwrap(),
        created_at:payment_history.created_at.unwrap()
    };
    list_payment_response.push(response);
    }
    Ok(list_payment_response)
}

async fn payment (payment_list: Vec<Payment>,data: &Arc<AppState>)-> Result<Vec<PaymentResponse>, (StatusCode, Json<serde_json::Value>)>{
    let mut list_payment_response: Vec<PaymentResponse> = Vec::new();
    for payment in payment_list  {
        let product = sqlx::query_as!(
            Product,
            r#"
            SELECT * FROM products
            WHERE id = $1
            "#,
            payment.id_product
        )
        .fetch_one(&data.db)
        .await
        .map_err(|e|error_response(format!("Failed to get product {:?}",e)
        .as_str()))?;
    let payment_response = payment_to_response(&payment, product_to_response(&product));
    list_payment_response.push(payment_response);
    }
    Ok(
        list_payment_response
    )
}
