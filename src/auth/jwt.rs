use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use jsonwebtoken::{encode, EncodingKey, Header};
use chrono::{Duration, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub sid: String,
    pub exp: usize,
}

pub fn generate_token(
    user_id: String,
    session_id: String,
) -> String {
    let claims = Claims {
        sub: user_id,
        sid: session_id,
        exp: (Utc::now() + Duration::hours(24)).timestamp() as usize,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(
            b"my_super_secret_key",
        ),
    ).unwrap()
}


pub fn verify_token(
    token: &str,
) -> Result<Claims, jsonwebtoken::errors::Error> {
    let data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(
            b"my_super_secret_key",
        ),
        &Validation::default(),
    )?;

    Ok(data.claims)
}