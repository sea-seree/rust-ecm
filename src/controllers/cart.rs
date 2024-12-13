use actix_web::{web, HttpResponse};
use uuid::Uuid;
use crate::services::cart_service;
use sea_orm::DatabaseConnection;
use serde_json::json;
use crate::error::ApiError;

#[derive(serde::Deserialize)]
pub struct AddToCartRequest {
    pub product_id: Uuid,
    pub quantity: i32,
}
/// เพิ่มสินค้าในตะกร้า
pub async fn add_to_cart(
    db: web::Data<DatabaseConnection>,
    user_id: web::Path<Uuid>,
    req: web::Json<AddToCartRequest>,
) -> Result<HttpResponse, ApiError> {
    let cart_item = cart_service::add_to_cart(
        &db,
        user_id.into_inner(),
        req.product_id,
        req.quantity,
    )
    .await?;
    Ok(HttpResponse::Ok().json(cart_item))
}

/// ลบสินค้าออกจากตะกร้า
pub async fn remove_from_cart(
    db: web::Data<DatabaseConnection>,
    path: web::Path<(Uuid, Uuid)>, // Tuple ของ (user_id, product_id)
) -> Result<HttpResponse, ApiError> {
    let (user_id, product_id) = path.into_inner();
    cart_service::remove_from_cart(&db, user_id, product_id).await?;
    Ok(HttpResponse::Ok().body("Item removed from cart"))
}

/// คำนวณราคารวมสินค้าในตะกร้า
pub async fn calculate_cart_total(
    db: web::Data<DatabaseConnection>,
    user_id: web::Path<Uuid>,
) -> Result<HttpResponse, ApiError> {
    let total_price = cart_service::calculate_cart_total(&db, user_id.into_inner()).await?;
    Ok(HttpResponse::Ok().json(json!({ "total_price": total_price })))
}

/// ล้างตะกร้าสินค้า
pub async fn clear_cart(
    db: web::Data<DatabaseConnection>,
    user_id: web::Path<Uuid>,
) -> Result<HttpResponse, ApiError> {
    cart_service::clear_cart(&db, user_id.into_inner()).await?;
    Ok(HttpResponse::Ok().body("Cart cleared successfully"))
}

/// ดึงรายการสินค้าในตะกร้า
pub async fn get_cart(
    db: web::Data<DatabaseConnection>,
    user_id: web::Path<Uuid>,
) -> Result<HttpResponse, ApiError> {
    let cart_items = cart_service::get_cart(&db, user_id.into_inner()).await?;
    Ok(HttpResponse::Ok().json(cart_items))
}