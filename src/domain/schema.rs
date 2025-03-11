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

#[derive(Serialize, Deserialize, Debug)]
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
pub struct CreatePaymentSchema{
    pub id_product: i32,
    pub product_amount: i16,
}
impl CreatePaymentSchema{
    pub fn extract( id_product: i32, product_amount: i16) -> Self {
        CreatePaymentSchema{
            id_product,
            product_amount
        }
    }
}

#[derive(Serialize, Deserialize,Debug)]
pub struct CreatePaymentHistorySchema{
    pub user_amount_money: i64,
    pub list_payment: Vec<CreatePaymentSchema>
}
impl CreatePaymentHistorySchema{
    pub fn extract(list_payment:Vec<CreatePaymentSchema>, user_amount_money: i64) -> Self {
        CreatePaymentHistorySchema{
            list_payment,
            user_amount_money,
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
#[derive(Serialize, Deserialize,Debug)]
pub struct Invoices {
    pub payment_method: Option<String>,
    pub invoice_url : String,
    pub expiry_date: String,
    pub status_history: Option<String>,
    pub status_payment: Option<String>,
}

#[derive(Serialize, Deserialize, Default,Debug)]
pub struct CreatedInvoices {
    pub external_id: String,
    pub items : Vec<InvoiceItem>,
    pub amount: i64,
    pub locale: String,
    pub payment_methods:Vec<String>,
    pub currency: String,
    pub invoice_duration: u32
}
impl CreatedInvoices{
    pub fn extract(external_id: String, items: Vec<InvoiceItem>, amount: i64) -> Self {
        CreatedInvoices{
            external_id,
            items,
            amount,
            payment_methods: vec![
                "OVO".to_string(), 
                "DANA".to_string(), 
                "SHOPEEPAY".to_string(),  
                "LINKAJA".to_string(),  
                "JENIUSPAY".to_string(),  
                "QRIS".to_string()
            ],
            currency: "IDR".to_string(),
            locale:"id".to_string(),
            invoice_duration:86400
        }
    }
}
#[derive(Serialize, Deserialize,Debug)]
pub struct InvoiceItem {
    pub name: String,
    pub price: Option<i32>,
    pub quantity: Option<i16>,
}
