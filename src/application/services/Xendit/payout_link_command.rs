use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Deserialize, Debug,Serialize)]
pub struct WebhookPayload {
    pub event: String,
    pub business_id: String,
    pub created: String,
    pub data: PayoutData,
}

#[derive(Deserialize, Debug,Serialize)]
pub struct PayoutData {
    pub id: String,
    pub created: String, 
    pub updated: String,
    pub reference_id: String,
    pub currency: String,
    pub business_id: String,
    pub expires_at: String,
    pub status: String,
    pub amount: f64,
    pub payout_link_url: String,
    pub recipient_email: String,
    pub recipient_phone_number: String,
}


pub async fn payout_link_command (
    Json(payload): Json<WebhookPayload>
) -> Result<impl IntoResponse,(StatusCode,Json<serde_json::Value>)>{
    println!("Payout Link {:?}",payload);
    Ok((
        StatusCode::CREATED,
        Json(json!({
            "status": "success",
            "message": "Payout link created",
            "payload":payload
        })),
    ))
}