use std::{collections::HashMap, fs::File, io::Write, sync::Arc};
use axum::{extract::{Multipart, State}, http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

use crate::{api::router::AppState, application::services::error_response::error_response, domain::models::product::{product_to_response, Product}};

pub async fn create_product_command(
    State(data): State<Arc<AppState>>,
    mut multipart: Multipart,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let mut form = HashMap::new();
    let mut image_path= None;

    // Iterate over the multipart fields
    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        if name == "product_image" {
            let file_name = field.file_name().unwrap().to_string();
            let file_data = field.bytes().await.unwrap();
            let file_path = format!("public/products/{}", file_name.replace(" ", "_"));
            let mut file = File::create(&file_path).map_err(|_|error_response("Failed to save image"))?;
            file.write_all(&file_data).map_err(|_|error_response("Failed to write image file"))?;
            image_path = Some(format!("/{}",file_path));
        }else{
            let data = field.text().await.unwrap();  // Get the text data of each field
            form.insert(name, data);
        }
    }

    // Try to extract the form data and handle potential errors
    let product_name = match form.get("product_name") {
        Some(value) => value.clone(),
        None => return Err(error_response("Missing product_name")),
    };
    
    let product_price = match form.get("product_price") {
        Some(value) => match value.parse::<i32>() {
            Ok(price) => price,
            Err(_) => return Err(error_response("Missing product_price")),
        },
        None => return Err(error_response("Missing product_price")),
    };

    let product_stock = match form.get("product_stock") {
        Some(value) => match value.parse::<i16>() {
            Ok(stock) => stock,
            Err(_) => return Err(error_response("Invalid product_stock")),
        },
        None => return Err(error_response("Invalid product_stock")),
    };

    let product_available = match form.get("product_available") {
        Some(value) => match value.parse::<bool>() {
            Ok(available) => available,
            Err(_) => return Err(error_response("Invalid product_available")),
        },
        None => return Err(error_response("Invalid product_available")),
    };

    let product_image = image_path.ok_or_else(||error_response("Missin Image Product"))?;

    let product = sqlx::query_as! (
        Product,
        r#"INSERT INTO products (product_name, product_image, product_price, product_stock, product_available)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING id, product_name, product_image, product_price, product_stock, product_available, created_at, updated_at
        "#,
        product_name,
        product_image,
        product_price,
        product_stock,
        product_available
    )
    .fetch_one(&data.db)
    .await
    .map_err(|_| {
        return error_response("Failed to create product")
    })?;
    
    // Assuming `result` is successful, return a success response

        Ok((
        StatusCode::CREATED,
        Json(json!({
            "status": "success",
            "message": "Product created successfully",
            "data": product_to_response(&product),
        })),
    ))

}

// Helper function to return an error response

