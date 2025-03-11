use std::env;

use crate::domain::schema::{CreatedInvoices, Invoices};

pub async fn make_invoices (
    invoices: CreatedInvoices,
    client: &reqwest::Client
) -> Result<Invoices,Box<dyn std::error::Error>>{
    let secret_key = env::var("XENDIT_SECRET_KEY").expect("XENDIT_SECRET_KEY must be set");
    let response = 
    client.post("https://api.xendit.co/v2/invoices")
    .json(&invoices)
    .basic_auth(secret_key, Some(""))
    .header("Content-Type", "application/json")
    .send()
    .await?;

    let invoice: Invoices = serde_json::from_str(response.text().await?.as_str())?;
    Ok(invoice)
}