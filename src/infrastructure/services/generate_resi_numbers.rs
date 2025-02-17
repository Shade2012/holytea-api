
use std::sync::Arc;

use axum::{http::StatusCode, response::IntoResponse, Extension, Json};
use rand::{Rng, SeedableRng};

use crate::{api::router::AppState, application::services::error_response::error_response};
//Result<impl IntoResponse,(StatusCode, Json<serde_json::Value>)>

pub async fn generate_resi_numbers(
    
    data: &Arc<AppState>,
) -> Result<String, (StatusCode, Json<serde_json::Value>)> {
    let mut rng = rand::rngs::StdRng::from_entropy();
    loop {
        let candidates: Vec<String> = (0..10) // Generate 10 candidates at once
        .map(|_| format!("AP-{}", rng.gen_range(10000000..=99999999)))
        .collect();

        let existing = sqlx::query_scalar!(
            r#"
            SELECT resi_number FROM payment_history WHERE resi_number = ANY($1)
            "#,
            &candidates
        ).fetch_all(&data.db).
        await
        .map_err(|_|{
            return error_response("Gagal Mengecheck Resi Number");
        })?;
        
        if let Some(unique) = candidates.into_iter().find(|r| !existing.contains(r)) {
        return Ok(unique);
        }
    }
}
    