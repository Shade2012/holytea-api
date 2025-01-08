use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub id: Option<i32>,
    pub username: String,
    pub email: String,
    pub password: Option<String>,
    pub created_at: Option<chrono::NaiveDateTime>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserResponse {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub created_at: chrono::NaiveDateTime,
}

pub fn user_to_response(user: &User) -> UserResponse {
    UserResponse {
        id: user.id.as_ref().unwrap().to_owned(),
        username: user.username.to_owned(),
        email: user.email.to_owned(),
        created_at: user.created_at.as_ref().unwrap().to_owned(),
    }
}
