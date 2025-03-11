use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Extension, Json};
use chrono::{DateTime, NaiveDateTime, Utc};
use crate::{api::router::AppState, application::services::{error_response::{self, error_response}, Xendit::make_invoices::make_invoices}, domain::{models::{payment::Payment, payment_history::{PaymentHistory, StatusHistory, StatusPayment}}, schema::{CreatePaymentHistorySchema, CreatePaymentSchema, CreatedInvoices, InvoiceItem, Invoices}}, infrastructure::services::generate_resi_numbers::generate_resi_numbers};


pub async fn create_payment_history_command (
    State(data): State<Arc<AppState>>,
    Extension(user_id): Extension<i32>,
    Json(payload): Json<CreatePaymentHistorySchema>
) -> Result<impl IntoResponse,(StatusCode,Json<serde_json::Value>)>{
    // Start a transaction to ensure atomicity
    let resi_number = generate_resi_numbers(&data).await;

let payment_history = sqlx::query_as!(
    PaymentHistory,
    r#"
    INSERT INTO payment_history (
        user_id, 
        user_amount_money, 
        total_price, 
        status_payment, 
        resi_number, 
        status_history
    )
    VALUES ($1, $2, $3, $4, $5, $6)
    RETURNING 
        id, 
        user_id, 
        user_amount_money, 
        invoice_url, 
        payment_method, 
        total_price, 
        status_payment AS "status_payment!: StatusPayment", 
        status_history AS "status_history?: StatusHistory", 
        invoice_expired,
        resi_number, 
        created_at, 
        updated_at
    "#,
    user_id,
    payload.user_amount_money,
    0,
    StatusPayment::Pending as StatusPayment,
    resi_number.unwrap(),
    None::<StatusHistory> as Option<StatusHistory>
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
        .map_err(|_|error_response("failed to sum total price"))?.sum;

        sqlx::query(
            r#"
        UPDATE payment_history
        SET total_price = $1
        WHERE id = $2
        "#,
        )
        .bind(total_price_payment.unwrap_or_default())
        .bind(payment_history.id.unwrap_or_default())
        .execute(&data.db).await.map_err(|e|error_response(
            format!("failed to update total price {}",e).as_str()
        )
        )?;
        let items:Vec<InvoiceItem> = list_items(&data,payment_history.id.unwrap()).await?;
        let invoices = CreatedInvoices::extract(
            payment_history.resi_number.unwrap(),
            items,
            total_price_payment.unwrap()
        );
        let invoices = make_invoices(invoices, &data.client).await.map_err(|e|error_response(
           format!("failed to make invoice {}",e).as_str()
        )
        )?;

        set_invoices_url(&data, payment_history.id.unwrap(), &invoices).await?;

        return Ok((StatusCode::CREATED, Json(serde_json::json!({
            "status": "success",
            "message": "Payment created successfully",
            "payout_url":invoices.invoice_url
        }))));
    }else{
        return Err(
            result.err().unwrap()
        );
    }
}


async fn list_items(
    data: &Arc<AppState>,
    payment_history_id:i32,
) -> Result<Vec<InvoiceItem>,(StatusCode,Json<serde_json::Value>)>{
    let items: Vec<InvoiceItem> = sqlx::query_as!(
        InvoiceItem,
        r#"
        SELECT pay.product_amount AS quantity, prod.product_price AS price, prod.product_name AS name
        FROM payment pay
        LEFT JOIN products AS prod ON pay.id_product = prod.id
        WHERE pay.id_payment_history = $1;
        "#,
        payment_history_id
    )
    .fetch_all(&data.db)
    .await
    .map_err(|_|error_response("failed to get items"))?;
    Ok(items)
}

async fn set_invoices_url(
    data: &Arc<AppState>,
    payment_history_id:i32,
    invoice: &Invoices
) -> Result<(),(StatusCode,Json<serde_json::Value>)>{
    let expired = convert_iso_to_chrono_naive_datetime(&invoice.expiry_date.as_str());
    _ = sqlx::query!(
        r#"
        UPDATE payment_history
        SET invoice_url = $1, invoice_expired = $2
        WHERE id = $3
        "#,
        invoice.invoice_url,
        expired,
        payment_history_id
    ).execute(&data.db).await.map_err(
    |_|error_response("failed to update invoice url")
    )?;
    Ok(())
}

fn convert_iso_to_chrono_naive_datetime(iso_date:&str) -> chrono::NaiveDateTime{
    let dt_uc = DateTime::parse_from_rfc3339(iso_date)
    .unwrap()
    .with_timezone(&Utc);
    let naive_datetime: NaiveDateTime = dt_uc.naive_utc();
    naive_datetime
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