use crate::services::product_service;
use actix_web::{web::{self}, HttpResponse};
use rust_decimal::Decimal;
use sea_orm::DatabaseConnection;
use serde::Deserialize;
use serde_with::{serde_as, FromInto};
use uuid::Uuid;
use crate::error::ApiError;

pub async fn get_products(
    db: web::Data<DatabaseConnection>,
) -> Result<HttpResponse, ApiError> {
    let products = product_service::get_all_products(&db).await?;
    Ok(HttpResponse::Ok().json(products))
}

pub async fn get_product(
    db: web::Data<DatabaseConnection>,
    product_id: web::Path<Uuid>,
) -> Result<HttpResponse, ApiError> {
    let product = product_service::get_product_by_id(&db, product_id.into_inner())
        .await?;

    Ok(HttpResponse::Ok().json(product))
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
) -> Result<HttpResponse, ApiError> {
    let valid_statuses = vec!["available", "reserved", "sold"];
    if let Some(status) = &data.status {
        if !valid_statuses.contains(&status.as_str()) {
            return Err(ApiError::ValidationError(format!(
                "Invalid status: {}",
                status
            )));
        }
    }

    let product = product_service::create_product(
        &db,
        data.name.clone(),
        data.description.clone(),
        data.price,
        data.status.clone(),
    )
    .await?;

    Ok(HttpResponse::Created().json(product))
}
pub async fn update_product(
    product_id: web::Path<Uuid>,
    data: web::Json<CreateProductRequest>,
    db: web::Data<DatabaseConnection>,
) -> Result<HttpResponse, ApiError> {
    let product = product_service::update_product(
        &db,
        product_id.into_inner(),
        Some(data.name.clone()),
        data.description.clone(),
        Some(data.price),
    )
    .await?;

    Ok(HttpResponse::Ok().json(product))
}
pub async fn delete_product(
    product_id: web::Path<Uuid>,
    db: web::Data<DatabaseConnection>,
) -> Result<HttpResponse, ApiError> {
    product_service::delete_product(&db, product_id.into_inner()).await?;
    Ok(HttpResponse::Ok().body("Product deleted successfully"))
}

#[derive(Deserialize)]
pub struct UpdateProductStatusRequest {
    pub status: String,
}

pub async fn update_product_status(
    product_id: web::Path<Uuid>,
    data: web::Json<UpdateProductStatusRequest>,
    db: web::Data<DatabaseConnection>,
) -> Result<HttpResponse, ApiError> {
    let valid_statuses = vec!["available", "reserved", "sold"];

    if !valid_statuses.contains(&data.status.as_str()) {
        return Err(ApiError::ValidationError(format!(
            "Invalid status: {}",
            data.status
        )));
    }

    product_service::update_product_status(&db, product_id.into_inner(), data.status.clone())
        .await?;

    Ok(HttpResponse::Ok().body("Product status updated successfully"))
}