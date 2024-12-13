use actix_web::{
    // dev::ServiceRequest,
    // error::ErrorUnauthorized,
    web,
    App,
    HttpServer,
    // Error,
    // HttpMessage,
};
use dotenv::dotenv;
use middleware::auth::AuthMiddleware;
use sea_orm::Database;
use std::env;

// lib
// use actix_web_httpauth::{extractors::bearer::BearerAuth, middleware::HttpAuthentication};
// use jsonwebtoken::{decode, DecodingKey, Validation};
// use actix_web::middleware::Condition;

mod controllers;
mod entity;
mod error;
mod middleware;
mod routes;
mod services;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = match env::var("DATABASE_URL") {
        Ok(url) => url,
        Err(_) => {
            eprintln!("DATABASE_URL must be set");
            std::process::exit(1);
        }
    };

    let db = match Database::connect(&database_url).await {
        Ok(conn) => conn,
        Err(e) => {
            eprintln!("Failed to connect to the database: {}", e);
            std::process::exit(1);
        }
    };

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db.clone()))
            .configure(routes::configure_auth_routes)
            .configure(routes::configure_product_routes)
            .configure(routes::configure_cart_routes)
            .wrap(AuthMiddleware)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

// async fn validator(req: ServiceRequest, credentials: BearerAuth) -> Result<ServiceRequest, (Error, ServiceRequest)> {
//     // ข้ามการตรวจสอบ token สำหรับ routes ลงทะเบียนและ login
//     if req.path().starts_with("/auth/register") || req.path().starts_with("/auth/login") {
//         return Ok(req);
//     }

//     // ตรวจสอบ token สำหรับ routes อื่นๆ
//     let token = credentials.token();
//     let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET not set");

//     match decode::<Claims>(
//         token,
//         &DecodingKey::from_secret(secret.as_bytes()),
//         &Validation::default(),
//     ) {
//         Ok(decoded) => {
//             req.extensions_mut().insert(decoded.claims.get_sub().to_string());
//             Ok(req)
//         }
//         Err(_) => Err((ErrorUnauthorized("Invalid token"), req))
//     }
// }
