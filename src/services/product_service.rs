use crate::entity::products;
use rust_decimal::Decimal;
use sea_orm::{ActiveModelTrait, DatabaseConnection, DbErr, EntityTrait, Set};

pub async fn get_all_products(db: &DatabaseConnection) -> Result<Vec<products::Model>, DbErr> {
    products::Entity::find().all(db).await
}

pub async fn get_product_by_id(
    db: &DatabaseConnection,
    product_id: uuid::Uuid,
) -> Result<Option<products::Model>, DbErr> {
    products::Entity::find_by_id(product_id).one(db).await
}

pub async fn create_product(
    db: &DatabaseConnection,
    name: String,
    description: Option<String>,
    price: Decimal,
    status: Option<String>,
) -> Result<products::Model, DbErr> {
    let new_product = products::ActiveModel {
        id: Set(uuid::Uuid::new_v4()),
        name: Set(name),
        description: Set(description),
        price: Set(price),
        status: Set(status.unwrap_or_else(|| "available".to_string())),
        created_at: Set(chrono::Utc::now()),
    };

    new_product.insert(db).await
}
pub async fn update_product(
    db: &DatabaseConnection,
    product_id: uuid::Uuid,
    name: Option<String>,
    description: Option<String>,
    price: Option<Decimal>, // ต้องเป็น Option เพราะอาจไม่ต้องการอัปเดต
) -> Result<products::Model, DbErr> {
    if let Some(product) = products::Entity::find_by_id(product_id).one(db).await? {
        let mut product: products::ActiveModel = product.into();
        if let Some(name) = name {
            product.name = Set(name);
        }
        if let Some(description) = description {
            product.description = Set(Some(description));
        }
        if let Some(price) = price {
            product.price = Set(price);
        }

        product.update(db).await
    } else {
        Err(DbErr::RecordNotFound("Product not found".to_string()))
    }
}

pub async fn update_product_status(
    db: &DatabaseConnection,
    product_id: uuid::Uuid,
    new_status: String,
) -> Result<(), DbErr> {
    if let Some(product) = products::Entity::find_by_id(product_id).one(db).await? {
        let mut active_model: products::ActiveModel = product.into();
        active_model.status = Set(new_status);
        active_model.update(db).await?;
    }
    Ok(())
}

pub async fn delete_product(db: &DatabaseConnection, product_id: uuid::Uuid) -> Result<(), DbErr> {
    if let Some(product) = products::Entity::find_by_id(product_id).one(db).await? {
        let active_model: products::ActiveModel = product.into(); // แปลง Model เป็น ActiveModel
        active_model.delete(db).await.map(|_| ()) // ลบ Record
    } else {
        Err(DbErr::RecordNotFound("Product not found".to_string()))
    }
}
