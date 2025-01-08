use std::env;

use axum::{body::{Body}, extract::Request, http::{header::{AUTHORIZATION}, StatusCode}, middleware::Next, response::{IntoResponse, Response}, Json};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};
use serde_json::{json};

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub exp: usize,  // Expiry time of the token
    pub iat: usize,  // Issued at time of the token
    pub id: i32,  // Email associated with the token
}

pub struct AuthError{
    pub status: StatusCode,
    pub message: String,
}
impl IntoResponse for AuthError{
    fn into_response(self) -> Response<Body>{
        let body = Json(json!({
            "status": self.status.as_u16(),
            "message": self.message,
        }));
        (self.status,body).into_response()
    }
}

pub async fn middleware(mut req:Request,next:Next)->Result<Response,AuthError>{
    let auth_header = req.headers_mut().get(AUTHORIZATION);
    let auth_header = match auth_header {
        Some(header) => header.to_str().map_err(|_|{
            AuthError{
                status: StatusCode::UNAUTHORIZED,
                message: "Invalid Token".to_string(),
            }
        })?,
        None => return Err(AuthError{
            status: StatusCode::UNAUTHORIZED,
            message: "No Token Provided".to_string(),
        }),
    };
    let mut header = auth_header.split_whitespace();
    println!("ini header {:?}",header);
    let (bearer,token)=(header.next(),header.next());
    if bearer != Some("Bearer") || token.is_none(){
        return Err(AuthError{
            status: StatusCode::UNAUTHORIZED,
            message: "Invalid Token".to_string(),
        });
    }
    let token_data = match decode_jwt(token.unwrap()).await{
        Ok(token_data) => token_data,
        Err(status) => return Err(AuthError{
            status,
            message: "Invalid Token".to_string(),
        }),
        
    };
    let claims = token_data.claims;
     // Attach the user ID to the request extensions
     req.extensions_mut().insert(claims.id);
     Ok(next.run(req).await)
}


pub async fn decode_jwt(token: &str) -> Result<TokenData<Claims>, StatusCode> {
    let secret_key = env::var("JWT_SECRET").expect("JWT_SECRET not set in environment");
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret_key.as_ref()),
        &Validation::default(),
    )
    .map_err(|_| StatusCode::UNAUTHORIZED)
}

pub async fn encode_jwt(id:i32) ->Result<String,StatusCode>{
    let now = Utc::now();
    let expire = now + Duration::days(31);
    let claim = Claims{
           iat: now.timestamp() as usize,
           exp: expire.timestamp() as usize,
           id
        };
        let secret_key = env::var("JWT_SECRET").expect("JWT_SECRET not set in environment");
    encode(&Header::default(), &claim, &EncodingKey::from_secret(secret_key.as_ref())
    ) .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}
