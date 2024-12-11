use crate::services::product_service;
use actix_web::{web, HttpResponse};
use rust_decimal::Decimal;
use sea_orm::DatabaseConnection;
use serde::Deserialize;
use serde_with::{serde_as, FromInto};
use uuid::Uuid;

pub async fn get_products(db: web::Data<DatabaseConnection>) -> HttpResponse {
    match product_service::get_all_products(&db).await {
        Ok(products) => HttpResponse::Ok().json(products),
        Err(_) => HttpResponse::InternalServerError().body("Error fetching products"),
    }
}

pub async fn get_product(
    db: web::Data<DatabaseConnection>,
    product_id: web::Path<uuid::Uuid>,
) -> HttpResponse {
    match product_service::get_product_by_id(&db, product_id.into_inner()).await {
        Ok(Some(product)) => HttpResponse::Ok().json(product),
        Ok(None) => HttpResponse::NotFound().body("Product not found"),
        Err(_) => HttpResponse::InternalServerError().body("Error fetching product"),
    }
}

#[serde_as]
#[derive(Deserialize)]
pub struct CreateProductRequest {
    name: String,
    description: Option<String>,
    #[serde_as(as = "FromInto<Decimal>")]
    price: Decimal,
    status: Option<String>,
}

pub async fn create_product(
    data: web::Json<CreateProductRequest>,
    db: web::Data<DatabaseConnection>,
) -> HttpResponse {
    let valid_statuses = vec!["available", "reserved", "sold"];
    if let Some(status) = &data.status {
        if !valid_statuses.contains(&status.as_str()) {
            return HttpResponse::BadRequest().body(format!("Invalid status: {}", status));
        }
    }
    match product_service::create_product(
        &db,
        data.name.clone(),
        data.description.clone(),
        data.price,
        data.status.clone(),
    )
    .await
    {
        Ok(product) => HttpResponse::Ok().json(product),
        Err(_) => HttpResponse::InternalServerError().body("Error creating product"),
    }
}
pub async fn update_product(
    product_id: web::Path<uuid::Uuid>,
    data: web::Json<CreateProductRequest>,
    db: web::Data<DatabaseConnection>,
) -> HttpResponse {
    match product_service::update_product(
        &db,
        product_id.into_inner(),
        Some(data.name.clone()),
        data.description.clone(),
        Some(data.price),
    )
    .await
    {
        Ok(update_product) => HttpResponse::Ok().json(update_product),
        Err(sea_orm::DbErr::RecordNotFound(err)) => HttpResponse::NotFound().body(err),
        Err(_) => HttpResponse::InternalServerError().body("Error updating product"),
    }
}
pub async fn delete_product(
    product_id: web::Path<Uuid>,
    db: web::Data<DatabaseConnection>,
) -> HttpResponse {
    match product_service::delete_product(&db, product_id.into_inner()).await {
        Ok(_) => HttpResponse::Ok().body("Product deleted successfully"),
        Err(sea_orm::DbErr::RecordNotFound(err)) => HttpResponse::NotFound().body(err),
        Err(_) => HttpResponse::InternalServerError().body("Error deleting product"),
    }
}

#[derive(Deserialize)]
pub struct UpdateProductStatusRequest {
    pub status: String,
}

pub async fn update_product_status(
    product_id: web::Path<Uuid>,
    data: web::Json<UpdateProductStatusRequest>,
    db: web::Data<DatabaseConnection>,
) -> HttpResponse {
    let valid_statuses = vec!["available", "reserved", "sold"];

    if !valid_statuses.contains(&&data.status.as_str()) {
        return HttpResponse::BadRequest().body(format!("Invalid status: {}", data.status));
    }

    match product_service::update_product_status(&db, product_id.into_inner(), data.status.clone())
        .await
    {
        Ok(_) => HttpResponse::Ok().body("Product status updated successfully"),
        Err(sea_orm::DbErr::RecordNotFound(err)) => HttpResponse::NotFound().body(err),
        Err(_) => HttpResponse::InternalServerError().body("Error updating product status"),
    }
}
