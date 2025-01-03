use crate::entity::users::{self, ActiveModel};
use crate::services::auth::{generate_jwt, hash_password, verify_password};
use actix_web::{web, HttpResponse};
use once_cell::sync::Lazy;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use serde::Deserialize;
use crate::error::ApiError;
use validator::{Validate, ValidationError};
use regex::Regex;

#[derive(Deserialize, Validate)]
pub struct RegisterData {
    #[validate(length(min = 1, max = 15, message = "must be 1 - 15 characters long"))]
    pub username: String,
    #[validate(email(message = "invalid"))]
    pub email: String,
    #[validate(custom(function = "validate_password", message = "invalid"), )]
    pub password: String,
}

fn validate_password(password: &str) -> Result<(), ValidationError> {
    static LOWERCASE_REGEX: Lazy<regex::Regex> = Lazy::new(|| {
        Regex::new(r"[a-z]").expect("Invalid lowercase regex")
    });
    static UPPERCASE_REGEX: Lazy<regex::Regex> = Lazy::new(|| {
        Regex::new(r"[A-Z]").expect("Invalid uppercase regex")
    });
    static DIGIT_REGEX: Lazy<regex::Regex> = Lazy::new(|| {
        Regex::new(r"\d").expect("Invalid digit regex")
    });

    // ตรวจสอบความยาวขั้นต่ำ
    if password.len() < 8 {
        return Err(ValidationError::new("Password must be at least 8 characters long"));
    }

    // ตรวจสอบตัวอักษรพิมพ์เล็ก
    if !LOWERCASE_REGEX.is_match(password) {
        return Err(ValidationError::new(
            "Password must contain at least one lowercase letter",
        ));
    }

    // ตรวจสอบตัวอักษรพิมพ์ใหญ่
    if !UPPERCASE_REGEX.is_match(password) {
        return Err(ValidationError::new(
            "Password must contain at least one uppercase letter",
        ));
    }

    // ตรวจสอบตัวเลข
    if !DIGIT_REGEX.is_match(password) {
        return Err(ValidationError::new("Password must contain at least one digit"));
    }

    Ok(())
}


pub async fn register(
    data: web::Json<RegisterData>,
    db: web::Data<DatabaseConnection>,
) -> Result<HttpResponse, ApiError> {
    match data.validate() {
        Ok(_) => (),
        Err(e) => return Err(ApiError::ValidationError(e.to_string())),
    }

    let hashed_password = hash_password(&data.password)?;
    let new_user = ActiveModel {
        id: Set(uuid::Uuid::new_v4()),
        username: Set(data.username.clone()),
        email: Set(data.email.clone()),
        hashed_password: Set(hashed_password),
        created_at: Set(chrono::Utc::now()),
        ..Default::default()
    };

//     if let Err(err) = new_user.insert(&**db).await {
//         println!("Error inserting user: {:?}", err); // Debug ข้อผิดพลาด
//         return HttpResponse::BadRequest().body("Failed to create user");
//     }

//     HttpResponse::Ok().body("User registered successfully")

    new_user
        .insert(&**db)
        .await
        .map_err(|_| ApiError::DatabaseError("Failed to create user".to_string()))?;

    Ok(HttpResponse::Ok().body("User registered successfully"))
}

#[derive(Deserialize)]
pub struct LoginData {
    pub username: String,
    pub password: String,
}

pub async fn login(data: web::Json<LoginData>, db: web::Data<DatabaseConnection>) -> Result<HttpResponse, ApiError> {
    if let Some(users) = users::Entity::find()
        .filter(users::Column::Username.eq(data.username.clone()))
        .one(&**db)
        .await
        .unwrap()
    {
        if verify_password(&data.password, &users.hashed_password)? {
            let token = generate_jwt(&users.id.to_string())?;
            return Ok(HttpResponse::Ok().json(token));
        }
    }

    Err(ApiError::AuthenticationError("Invalid credentials".to_string()))
}
