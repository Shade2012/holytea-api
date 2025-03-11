use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use super::payment::PaymentResponse;

#[derive(Debug,Clone,PartialEq,PartialOrd,sqlx::Type,Deserialize,Serialize)]
#[sqlx(type_name = "status_payment_enum")]
pub enum StatusPayment {
    Selesai,
    Gagal,
    Pending
}
impl StatusPayment{
    pub fn convert_payment(data:&str) -> StatusPayment{
        match data {
            "Selesai" => StatusPayment::Selesai,
            "Gagal" =>  StatusPayment::Gagal,
            "Pending" => StatusPayment::Pending,
            _ => StatusPayment::Pending
        }
    }
}

#[derive(Debug,Clone,PartialEq,PartialOrd,sqlx::Type,Deserialize,Serialize)]
#[sqlx(type_name = "status_history_enum")]
pub enum StatusHistory {
    Selesai,
    Gagal,
    SedangDiproses
}
impl StatusHistory{
    pub fn convert_history(data:&str) -> StatusHistory{
        match data {
            "Selesai" => StatusHistory::Selesai,
            "Gagal" =>  StatusHistory::Gagal,
            "Sedang Diproses" => StatusHistory::SedangDiproses,
            _ => StatusHistory::SedangDiproses
        }
    }
    pub fn convert_history_to_string(&self) -> String{
        match self {
            StatusHistory::Selesai => "Selesai".to_string(),
            StatusHistory::Gagal => "Gagal".to_string(),
            StatusHistory::SedangDiproses => "Sedang Diproses".to_string(),
        }
    }

}

#[derive(Debug,Deserialize, Serialize,FromRow)]
#[allow(non_snake_case)]
pub struct PaymentHistory{
    pub id: Option<i32>,
    pub resi_number: Option<String>,
    pub user_id: i32,
    pub user_amount_money: i64,
    pub total_price: i64,
    pub status_payment: StatusPayment,
    pub payment_method: Option<String>,
    pub invoice_url: Option<String>,
    pub status_history: Option<StatusHistory>,
    pub invoice_expired: Option<chrono::NaiveDateTime>,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>
}

#[derive(Debug, sqlx::FromRow, Deserialize, Serialize)]
pub struct PaymentHistoryResponse{
    pub id: i32,
    pub resi_number: String,
    pub status_payment: StatusPayment,
    pub payment_method: Option<String>,
    pub invoice_url: Option<String>,
    pub status_history: Option<StatusHistory>,
    pub user_id: i32,
    pub user_amount_money: i64,
    pub total_price: i64,
    pub product_payment: Vec<PaymentResponse>,
    pub invoice_expired: Option<chrono::NaiveDateTime>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime
}

pub fn payment_history_to_response(payment_history: &PaymentHistory) -> PaymentHistoryResponse{
    PaymentHistoryResponse{
        resi_number: String::from(payment_history.resi_number.as_ref().unwrap().clone()),
        id:payment_history.id.unwrap_or(0),
        status_payment:payment_history.status_payment.to_owned(),
        status_history:payment_history.status_history.as_ref().cloned(),
        total_price:payment_history.total_price,
        payment_method:payment_history.payment_method.as_ref().cloned(),
        invoice_url: payment_history.invoice_url.as_ref().cloned(),
        invoice_expired:payment_history.invoice_expired,
        user_amount_money:payment_history.user_amount_money,
        user_id:payment_history.user_id,
        product_payment:vec![],
        created_at:payment_history.created_at.unwrap_or_default(),
        updated_at:payment_history.updated_at.unwrap_or_default(),
    }
}

pub fn payment_history_to_response_list(payment_history: &PaymentHistory, list_payment:Vec<PaymentResponse>) -> PaymentHistoryResponse{
    PaymentHistoryResponse{
        resi_number: payment_history.resi_number.as_ref().unwrap().clone(),
        id:payment_history.id.unwrap_or(0),
        status_payment:payment_history.status_payment.to_owned(),
        status_history:payment_history.status_history.as_ref().cloned(),
        total_price:payment_history.total_price,
        payment_method:payment_history.payment_method.as_ref().cloned(),
        invoice_url: payment_history.invoice_url.as_ref().cloned(),
        invoice_expired:payment_history.invoice_expired,
        user_amount_money:payment_history.user_amount_money,
        user_id:payment_history.user_id,
        product_payment:list_payment,
        created_at:payment_history.created_at.unwrap_or_default(),
        updated_at:payment_history.updated_at.unwrap_or_default(),
    }
}