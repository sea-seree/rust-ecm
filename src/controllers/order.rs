use actix_web::{web, HttpResponse};
use uuid::Uuid;
use crate::{services::order_service, error::ApiError};
use sea_orm::DatabaseConnection;

pub async fn create_order(
    db: web::Data<DatabaseConnection>,
    user_id: web::Path<Uuid>,
) -> Result<HttpResponse, ApiError> {
    let order = order_service::create_order(&db, user_id.into_inner()).await?;
    Ok(HttpResponse::Ok().json(order))
}

pub async fn get_order_details(
    db: web::Data<DatabaseConnection>,
    order_id: web::Path<Uuid>,
) -> Result<HttpResponse, ApiError> {
    let (order, items) = order_service::get_order_details(&db, order_id.into_inner()).await?;
    Ok(HttpResponse::Ok().json((order, items)))
}

pub async fn get_order_history(
    db: web::Data<DatabaseConnection>,
    user_id: web::Path<Uuid>,
) -> Result<HttpResponse, ApiError> {
    let orders = order_service::get_order_history(&db, user_id.into_inner()).await?;
    Ok(HttpResponse::Ok().json(orders))
}

pub async fn update_order_status(
    db: web::Data<DatabaseConnection>,
    order_id: web::Path<Uuid>,
    new_status: web::Json<String>,
) -> Result<HttpResponse, ApiError> {
    order_service::update_order_status(&db, order_id.into_inner(), new_status.into_inner()).await?;
    Ok(HttpResponse::Ok().body("Order status updated successfully"))
}
