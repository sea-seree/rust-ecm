use bcrypt::{hash, verify};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use std::env;
use crate::error::ApiError;

#[derive(Serialize, Deserialize)]
pub struct Claims {
    sub: String,
    exp: usize,
}

impl Claims {
    pub fn get_sub(&self) -> &str {
        &self.sub
    }
}

pub fn hash_password(password: &str) -> Result<String, ApiError> {
    hash(password, 4).map_err(|_| ApiError::InternalServerError)
}

pub fn verify_password(password: &str, hashed: &str) -> Result<bool, ApiError> {
    verify(password, hashed).map_err(|_| ApiError::InternalServerError)
}

pub fn generate_jwt(user_id: &str) -> Result<String, ApiError>{
    let secret = env::var("JWT_SECRET").map_err(|_| ApiError::InternalServerError)?;
    let claims = Claims {
        sub: user_id.to_owned(),
        exp: chrono::Utc::now().timestamp() as usize + 3600, // 1 ชั่วโมง
    };
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|_| ApiError::InternalServerError)
}
