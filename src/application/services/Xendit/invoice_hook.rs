use std::sync::Arc;

use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};

use crate::api::router::AppState;

#[derive(Deserialize, Debug,Serialize)]
// "EXPIRED"
// "PAID"
pub struct WebhookInvoice {
    pub id: String,
    pub external_id: String,
    pub payment_method: Option<String>,
    pub payment_channel: Option<String>,
    pub status: String,
    pub merchant_name: String,
    pub amount: i64,
    pub paid_amount:Option<i64>,
    pub created: String,
    pub updated: String,
    pub currency: String,
}
pub async fn invoice_hook (
    State(data): State<Arc<AppState>>,
    Json(payload): Json<WebhookInvoice>
){
    if payload.status == "PAID" {
        let _ = sqlx::query!(
            r#"
            UPDATE payment_history 
            SET status_payment = 'Selesai', payment_method = $1
            WHERE resi_number = $2
            "#,
            payload.payment_channel,
            payload.external_id
        )
        .execute(&data.db)
        .await.map_err(|e|{
            println!("error update status 'selesai' payment history {:#?}",e);
        });
    } else{
        let _ = sqlx::query!(
            r#"
            UPDATE payment_history 
            SET status_payment = 'Gagal'
            WHERE resi_number = $1
            "#,
            payload.external_id
        )
        .execute(&data.db)
        .await.map_err(|e|{
            println!("error update status 'gagal' payment history {:#?}",e);
        });
    }
}





