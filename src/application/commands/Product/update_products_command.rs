use std::{collections::HashMap, fs::File, io::Write, sync::Arc};

use axum::{extract::{multipart, Multipart, Path, State}, http::StatusCode, response::IntoResponse, Json};
use sqlx::query_as;

use crate::{api::router::AppState, application::services::error_response::error_response, domain::{models::product::{product_to_response, Product}, schema::UpdateProductSchema}};

pub async fn update_products_command(
    State(data): State<Arc<AppState>>,
    Path(id): Path<i32>,
    mut multipart : Multipart,
)-> Result<impl IntoResponse,(StatusCode,Json<serde_json::Value>)> {
    let mut form = HashMap::new();
    let mut image_path = None;
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
    let product_name: Option<String> = match form.get("product_name") {
        Some(value) => Some(value.clone()),
        None => None,
    };

    let product_price: Option<i32> = match form.get("product_price") {
        Some(value) => value.parse::<i32>().ok(),
        None => None,  
    };

    let product_stock: Option<i16> = match form.get("product_stock") {
        Some(value) => value.parse::<i16>().ok(),
        None => None,  
    };

    let product_available: Option<bool> = match form.get("product_available") {
        Some(value) => value.parse::<bool>().ok(),
        None => None,  
    };
    let product_image = match image_path {
        Some(value) => Some(value),
        None => None,
    };
    let schema =UpdateProductSchema::extract(
        product_name,
        product_image,
        product_price,
        product_stock,
        product_available
    );
    let product = query_as!(
        Product,
        r#"
        UPDATE products SET
            product_name = COALESCE($1, product_name),
            product_image = COALESCE($2, product_image),
            product_price = COALESCE($3, product_price),
            product_stock = COALESCE($4, product_stock),
            product_available = COALESCE($5, product_available)
        WHERE id = $6
        RETURNING *
        "#
        ,schema.product_name
        ,schema.product_image
        ,schema.product_price
        ,schema.product_stock
        ,schema.product_available
        ,id
    ).fetch_one(&data.db)
    .await.map_err(|_|{
        return error_response("Failed to update product")
    })?;

    Ok(
        (
            StatusCode::OK,
            Json(serde_json::json!({
                "status": "success",
                "message": "Product updated successfully",
                "data": product_to_response(&product)
            }))
        )
    )
    
}