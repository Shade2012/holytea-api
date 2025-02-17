use std::sync::Arc;

use axum::{extract::{Query, State}, http::StatusCode, response::IntoResponse, Json};

use crate::{api::router::AppState, application::services::error_response::error_response, domain::models::product::Product};

pub async fn testing (
    State(data): State<Arc<AppState>>,
    Query(params): Query<Vec<(String, String)>>,
) -> Result<impl IntoResponse,(StatusCode,Json<serde_json::Value>)>{
    let mut query = String::from("SELECT * FROM products");
    if !params.is_empty(){
        let new_query = format!("{} WHERE ", query);
        query = new_query;
        for (index,(key,value)) in params.iter().enumerate(){
            if index == 0 {
                query = format!("{} {} = '{}'",query,key,value);
            }else{
                query = format!("{} AND {} = '{}'",query,key,value);
            }
        }
    }
    let result: Vec<Product> = sqlx::query_as(
        query.as_str()
    )
    .fetch_all(&data.db)
    .await
    .map_err(
        |e|error_response(format!("Failed to get product {:?}",e).as_str())
    )?;
    println!("Query {}",query);
    Ok(
        (
            StatusCode::OK,
            Json(serde_json::json!({
                "status":"success",
                "message":"Successfully get all product",
                "data": result
            }))
        )
    )
}