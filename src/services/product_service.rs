use crate::entity::products;
use rust_decimal::Decimal;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set};
use uuid::Uuid;
use crate::error::ApiError;


pub async fn get_all_products(db: &DatabaseConnection) -> Result<Vec<products::Model>, ApiError> {
    products::Entity::find()
        .all(db)
        .await
        .map_err(|_| ApiError::DatabaseError("Failed to fetch products".to_string()))
}

pub async fn get_product_by_id(
    db: &DatabaseConnection,
    product_id: Uuid,
) -> Result<products::Model, ApiError> {
    products::Entity::find_by_id(product_id)
        .one(db)
        .await?
        .ok_or_else(|| ApiError::NotFound(format!("Product with ID {} not found", product_id)))
}

pub async fn create_product(
    db: &DatabaseConnection,
    name: String,
    description: Option<String>,
    price: Decimal,
    status: Option<String>,
) -> Result<products::Model, ApiError> {
    let new_product = products::ActiveModel {
        id: Set(Uuid::new_v4()),
        name: Set(name),
        description: Set(description),
        price: Set(price),
        status: Set(status.unwrap_or_else(|| "available".to_string())),
        created_at: Set(chrono::Utc::now()),
    };

    new_product
        .insert(db)
        .await
        .map_err(|_| ApiError::DatabaseError("Failed to create product".to_string()))
}
pub async fn update_product(
    db: &DatabaseConnection,
    product_id: Uuid,
    name: Option<String>,
    description: Option<String>,
    price: Option<Decimal>,
) -> Result<products::Model, ApiError> {
    let product = products::Entity::find_by_id(product_id)
        .one(db)
        .await?
        .ok_or_else(|| ApiError::NotFound(format!("Product with ID {} not found", product_id)))?;

    let mut active_model: products::ActiveModel = product.into();
    if let Some(name) = name {
        active_model.name = Set(name);
    }
    if let Some(description) = description {
        active_model.description = Set(Some(description));
    }
    if let Some(price) = price {
        active_model.price = Set(price);
    }

    active_model
        .update(db)
        .await
        .map_err(|_| ApiError::DatabaseError("Failed to update product".to_string()))
}

pub async fn update_product_status(
    db: &DatabaseConnection,
    product_id: Uuid,
    new_status: String,
) -> Result<(), ApiError> {
    let product = products::Entity::find_by_id(product_id)
        .one(db)
        .await?
        .ok_or_else(|| ApiError::NotFound(format!("Product with ID {} not found", product_id)))?;

    let mut active_model: products::ActiveModel = product.into();
    active_model.status = Set(new_status);
    active_model
        .update(db)
        .await
        .map_err(|_| ApiError::DatabaseError("Failed to update product status".to_string()))?;

    Ok(())
}

pub async fn delete_product(db: &DatabaseConnection, product_id: Uuid) -> Result<(), ApiError> {
    let product = products::Entity::find_by_id(product_id)
        .one(db)
        .await?
        .ok_or_else(|| ApiError::NotFound(format!("Product with ID {} not found", product_id)))?;

    let active_model: products::ActiveModel = product.into();
    active_model
        .delete(db)
        .await
        .map_err(|_| ApiError::DatabaseError("Failed to delete product".to_string()))?;

    Ok(())
}
