use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use std::env;
use sea_orm::Database;

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
        App::new()
            .app_data(web::Data::new(db.clone())) 
            .configure(routes::configure_auth_routes)
            .configure(routes::configure_product_routes)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
