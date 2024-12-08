use actix_web::{web, HttpResponse};
use serde::Deserialize;
use crate::services::auth::{hash_password, verify_password, generate_jwt};
use crate::entity::users::{self, ActiveModel};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};

#[derive(Deserialize)]
pub struct RegisterData {
    pub username: String,
    pub email: String,
    pub password: String,
}

pub async fn register(data: web::Json<RegisterData>, db: web::Data<DatabaseConnection>) -> HttpResponse {
    let hashed_password = hash_password(&data.password);
    let new_user = ActiveModel {
        id: Set(uuid::Uuid::new_v4()),
        username: Set(data.username.clone()),
        email: Set(data.email.clone()),
        hashed_password: Set(hashed_password),
        ..Default::default()
    };

    if let Err(err) = new_user.insert(&**db).await {
        println!("Error inserting user: {:?}", err); // Debug ข้อผิดพลาด
        return HttpResponse::BadRequest().body("Failed to create user");
    }

    HttpResponse::Ok().body("User registered successfully")
}

#[derive(Deserialize)]
pub struct LoginData {
    pub username: String,
    pub password: String,
}

pub async fn login(data: web::Json<LoginData>, db: web::Data<DatabaseConnection>) -> HttpResponse {
    if let Some(users) = users::Entity::find()
        .filter(users::Column::Username.eq(data.username.clone()))
        .one(&**db)
        .await
        .unwrap()
    {
        if verify_password(&data.password, &users.hashed_password) {
            let token = generate_jwt(&users.id.to_string());
            return HttpResponse::Ok().json(token);
        }
    }

    HttpResponse::Unauthorized().body("Invalid credentials")
}
