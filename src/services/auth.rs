use bcrypt::{hash, verify};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use std::env;

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

pub fn hash_password(password: &str) -> String {
    hash(password, 4).expect("Failed to hash password")
}

pub fn verify_password(password: &str, hashed: &str) -> bool {
    match verify(password, hashed) {
        Ok(valid) => valid,
        Err(err) => {
            println!("Error verifying password: {:?}", err); // Log ข้อผิดพลาด
            false
        }
    }
}

pub fn generate_jwt(user_id: &str) -> String {
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET not set");
    let claims = Claims {
        sub: user_id.to_owned(),
        exp: chrono::Utc::now().timestamp() as usize + 3600, // 1 ชั่วโมง
    };
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .expect("Failed to generate token")
}
