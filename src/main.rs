use actix_web::{
    // dev::ServiceRequest, 
    // error::ErrorUnauthorized, 
    web, App, HttpServer,
    // Error, 
    // HttpMessage, 
    };
use dotenv::dotenv;
use std::env;
use sea_orm::Database;
use middleware::auth::AuthMiddleware;

// lib
// use actix_web_httpauth::{extractors::bearer::BearerAuth, middleware::HttpAuthentication};
// use jsonwebtoken::{decode, DecodingKey, Validation};
// use actix_web::middleware::Condition;

mod middleware;
mod routes;
mod services;
mod controllers;
mod entity;



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db = Database::connect(&database_url).await.unwrap();

    HttpServer::new(move || {
        // let auth = HttpAuthentication::bearer(validator);
        App::new()
            .app_data(web::Data::new(db.clone()))
            .configure(routes::configure_auth_routes)
            .configure(routes::configure_product_routes)
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