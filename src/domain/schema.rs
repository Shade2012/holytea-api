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