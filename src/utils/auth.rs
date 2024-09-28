use axum::{
    extract::Request,
    http::{Response, StatusCode},
    middleware::Next,
};
use chrono::{Duration, Utc};
use jsonwebtoken::{
    decode, encode, DecodingKey, EncodingKey, Header, Validation,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub exp: usize,
    pub iat: usize,
    pub email: String,
}

pub fn is_valid_token(token: &str, secret: &str) -> bool {
    let key = DecodingKey::from_secret(secret.as_bytes());

    match decode::<Claims>(token, &key, &Validation::default()) {
        Ok(_) => true,
        Err(e) => {
            println!("Token validation error: {:?}", e);
            false
        }
    }
}

pub async fn auth(request: Request, next: Next) -> Response {
    let response = next.run(request).await;
    response
}

pub fn encode_jwt(email: String, secret: &str) -> Result<String, StatusCode> {
    let now = Utc::now();
    let expire: chrono::TimeDelta = Duration::days(14);
    let exp: usize = (now + expire).timestamp() as usize;
    let iat: usize = now.timestamp() as usize;
    let claim = Claims { iat, exp, email };

    encode(
        &Header::default(),
        &claim,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}
