use serde::{Deserialize, Serialize};

use super::payment::PaymentResponse;
#[derive(Debug,Clone,PartialEq,PartialOrd,sqlx::Type,Deserialize,Serialize)]
#[sqlx(type_name = "status_payment_enum")]
pub enum StatusPayment {
    Selesai,
    Gagal,
    Pending
}

#[derive(Debug, sqlx::FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct PaymentHistory{
    pub id: Option<i32>,
    pub user_id: i32,
    pub user_amount_money: i64,
    pub total_price: i64,
    pub status_payment: StatusPayment,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>
}

#[derive(Debug, sqlx::FromRow, Deserialize, Serialize)]
pub struct PaymentHistoryResponse{
    pub id: i32,
    pub user_id: i32,
    pub user_amount_money: i64,
    pub total_price: i64,
    pub product_payment: Vec<PaymentResponse>,
    pub status_payment: StatusPayment,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime
}

pub fn payment_history_to_response(payment_history: &PaymentHistory) -> PaymentHistoryResponse{
    PaymentHistoryResponse{
        id:payment_history.id.unwrap_or(0),
        status_payment:payment_history.status_payment.to_owned(),
        total_price:payment_history.total_price,
        user_amount_money:payment_history.user_amount_money,
        user_id:payment_history.user_id,
        product_payment:vec![],
        created_at:payment_history.created_at.unwrap_or_default(),
        updated_at:payment_history.updated_at.unwrap_or_default(),
    }
}

pub fn payment_history_to_response_list(payment_history: &PaymentHistory, list_payment:Vec<PaymentResponse>) -> PaymentHistoryResponse{
    PaymentHistoryResponse{
        id:payment_history.id.unwrap_or(0),
        status_payment:payment_history.status_payment.to_owned(),
        total_price:payment_history.total_price,
        user_amount_money:payment_history.user_amount_money,
        user_id:payment_history.user_id,
        product_payment:list_payment,
        created_at:payment_history.created_at.unwrap_or_default(),
        updated_at:payment_history.updated_at.unwrap_or_default(),
    }
}