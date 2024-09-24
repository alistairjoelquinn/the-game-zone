use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
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
