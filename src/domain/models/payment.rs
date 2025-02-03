use serde::{Deserialize, Serialize};

use super::product::ProductResponse;

#[derive(Debug,Deserialize,Serialize,Clone)]
pub struct Payment{
    pub id:Option<i32>,
    pub id_payment_history:i32,
    pub id_product:i32,
    pub payment_product_price:i32,
    pub product_amount:i16
}

#[derive(Deserialize, Serialize,Debug,Clone)]
pub struct PaymentResponse{
    pub id:i32,
    pub id_payment_history:i32,
    pub id_product:i32,
    pub product:ProductResponse,
    pub payment_product_price:i32,
    pub product_amount:i16
}

pub fn payment_to_response(payment: &Payment,product_response: ProductResponse) -> PaymentResponse{
    PaymentResponse {
        id: payment.id.unwrap_or_default(),
        id_payment_history: payment.id_payment_history,
        id_product:payment.id_product,
        product:product_response,
        payment_product_price: payment.payment_product_price,
        product_amount: payment.product_amount 
    }
}