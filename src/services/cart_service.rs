use sea_orm::{entity::*, query::*, DatabaseConnection};
use uuid::Uuid;
use rust_decimal::Decimal;
use crate::entity::{cart, products};
use crate::error::ApiError;


 pub async fn add_to_cart(
    db: &DatabaseConnection,
    user_id: Uuid,
    product_id: Uuid,
    quantity: i32,
) -> Result<cart::Model, ApiError> {
    // หาสินค้าในตะกร้าที่มีอยู่
    let existing_item = cart::Entity::find()
        .filter(cart::Column::UserId.eq(user_id))
        .filter(cart::Column::ProductId.eq(product_id))
        .one(db)
        .await
        .map_err(ApiError::from)?;

    match existing_item {
        Some(item) => {
            // กรณีมีสินค้าในตะกร้าอยู่แล้ว
            let mut active_model: cart::ActiveModel = item.into();
            let current_quantity = match &active_model.quantity {
                ActiveValue::Set(value) => *value,
                _ => 0,
            };
            active_model.quantity = Set(current_quantity + quantity);
            active_model.update(db).await.map_err(ApiError::from)
        }
        None => {
            // กรณีไม่มีสินค้าในตะกร้า
            let new_cart_item = cart::ActiveModel {
                id: Set(Uuid::new_v4()),
                user_id: Set(user_id),
                product_id: Set(product_id),
                quantity: Set(quantity),
            };
            new_cart_item.insert(db).await.map_err(ApiError::from)
        }
    }
}
/// ลบสินค้าออกจากตะกร้า
pub async fn remove_from_cart(
    db: &DatabaseConnection,
    user_id: Uuid,
    product_id: Uuid,
) -> Result<(), ApiError> {
    let item_to_remove = cart::Entity::find()
        .filter(cart::Column::UserId.eq(user_id))
        .filter(cart::Column::ProductId.eq(product_id))
        .one(db)
        .await
        .map_err(ApiError::from)?;

    if let Some(item) = item_to_remove {
        let item: cart::ActiveModel = item.into();
        item.delete(db).await.map_err(ApiError::from)?;
    }

    Ok(())
}
pub async fn clear_cart(
    db: &DatabaseConnection, 
    user_id: Uuid
) -> Result<(), ApiError> {
    // ลบทุกรายการในตะกร้าของผู้ใช้รายนี้
    cart::Entity::delete_many()
        .filter(cart::Column::UserId.eq(user_id))
        .exec(db)
        .await
        .map_err(ApiError::from)?;

    Ok(())
}

/// ดึงรายการสินค้าในตะกร้าทั้งหมดของผู้ใช้
pub async fn get_cart(
    db: &DatabaseConnection, 
    user_id: Uuid
) -> Result<Vec<cart::Model>, ApiError> {
    let cart_items = cart::Entity::find()
        .filter(cart::Column::UserId.eq(user_id))
        .all(db)
        .await
        .map_err(ApiError::from)?;

    Ok(cart_items)
}

/// คำนวณราคารวมสินค้าในตะกร้า
pub async fn calculate_cart_total(
    db: &DatabaseConnection,
    user_id: Uuid,
) -> Result<Decimal, ApiError> {
    let cart_items = cart::Entity::find()
        .filter(cart::Column::UserId.eq(user_id))
        .all(db)
        .await
        .map_err(ApiError::from)?;

    let mut total_price = Decimal::new(0, 0);

    for item in cart_items {
        if let Some(product) = products::Entity::find_by_id(item.product_id)
            .one(db)
            .await
            .map_err(ApiError::from)?
        {
            total_price += product.price * Decimal::from(item.quantity);
        } else {
            return Err(ApiError::NotFound(format!(
                "Product with ID {} not found",
                item.product_id
            )));
        }
    }

    Ok(total_price)
}
