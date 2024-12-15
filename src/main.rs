use actix_web::{
    web,
    App,
    HttpServer,
};
use dotenv::dotenv;
use middleware::auth::AuthMiddleware;
use sea_orm::Database;
use std::env;

mod controllers;
mod entity;
mod error;
mod middleware;
mod routes;
mod services;
mod config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let app_config = match config::AppConfig::new() {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("Failed to load configuration: {}", e);
            std::process::exit(1);
        }
    };

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
            .app_data(web::Data::new(app_config.clone()))
            .configure(routes::configure_auth_routes)
            .configure(routes::configure_product_routes)
            .configure(routes::configure_cart_routes)
            .configure(routes::configure_order_routes)
            .wrap(AuthMiddleware)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}