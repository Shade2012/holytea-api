use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Default)]
pub struct CreateUserSchema {
    pub username: String,
    pub email: String,
    pub password: String
}


#[derive(Serialize, Deserialize, Default)]
pub struct LoginUserSchema{
    pub email:String,
    pub password:String
}

#[derive(Serialize, Deserialize, Default,Debug)]
pub struct CreateProductSchema{
    pub product_name: String,
    pub product_image: String,
    pub product_price: i32,
    pub product_stock: i16,
    pub product_available: bool,
}
impl CreateProductSchema{
    pub fn extract(product_name: String, product_image: String, product_price: i32, product_stock: i16, product_available: bool) -> Self {
        CreateProductSchema{
            product_name,
            product_image,
            product_price,
            product_stock,
            product_available
        }
    }
}

#[derive(Serialize, Deserialize, Default,Debug)]
pub struct UpdateProductSchema{
    pub product_name: Option<String>,
    pub product_image: Option<String>,
    pub product_price: Option<i32>,
    pub product_stock: Option<i16>,
    pub product_available: Option<bool>,
}

impl UpdateProductSchema{
    pub fn extract(product_name: Option<String>, product_image:  Option<String>, product_price: Option<i32>, product_stock: Option<i16>, product_available: Option<bool>) -> Self {
        UpdateProductSchema{
            product_name,
            product_image,
            product_price,
            product_stock,
            product_available
        }
    }
}

