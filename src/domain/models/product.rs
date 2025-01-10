use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Product {
    pub id: Option<i32>,
    pub product_name: String,
    pub product_image: Option<String>,
    pub product_price: i32,
    pub product_stock: i16,
    pub product_available: bool,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProductResponse {
    pub id: i32,
    pub product_name: String,
    pub product_image: String,
    pub product_price: i32,
    pub product_stock: i16,
    pub product_available: bool,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

pub fn product_to_response(product: &Product) -> ProductResponse {
   ProductResponse {
        id: product.id.as_ref().unwrap().to_owned(),
        product_name: product.product_name.to_owned(),
        product_image: format!("http://localhost:8080{}",product.product_image.as_ref().unwrap().to_owned()),
        product_price: product.product_price.to_owned(),
        product_stock: product.product_stock.to_owned(),
        product_available: product.product_available.to_owned(),
        created_at: product.created_at.as_ref().unwrap().to_owned(),
        updated_at: product.updated_at.as_ref().unwrap().to_owned(),
    }
}
