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

pub async fn auth(request: Request, next: Next) -> Response {
    println!(
        "
            {} --------- Received request: {} {}",
        crate::utils::get_time().format("%Y-%m-%d %H:%M:%S"),
        request.method(),
        request.uri(),
    );

    let response = next.run(request).await;
    response
}

pub fn encode_jwt(email: String) -> Result<String, StatusCode> {
    let secret: String = "randomStringTypicallyFromEnv".to_string();
    let now = Utc::now();
    let expire: chrono::TimeDelta = Duration::hours(24);
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
