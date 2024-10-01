use axum::http::StatusCode;
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Claims {
    pub exp: usize,
    pub iat: usize,
    pub username: String,
}

pub fn encode_jwt(
    username: String,
    secret: &str,
) -> Result<String, StatusCode> {
    let now = Utc::now();
    let expire: chrono::TimeDelta = Duration::days(14);
    let exp: usize = (now + expire).timestamp() as usize;
    let iat: usize = now.timestamp() as usize;
    let claim = Claims { iat, exp, username };

    encode(
        &Header::default(),
        &claim,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}
