use sea_orm::{Database, DbErr};
use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() -> Result<(), DbErr> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    
    println!("Attempting to connect to: {}", database_url);
    
    match Database::connect(&database_url).await {
        Ok(_) => {
            println!("✅ Successfully connected to database!");
            Ok(())
        }
        Err(e) => {
            eprintln!("❌ Connection failed: {:?}", e);
            Err(e)
        }
    }
}