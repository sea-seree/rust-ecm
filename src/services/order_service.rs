use crate::entity::{cart, order_items, orders, products};
use rust_decimal::Decimal;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use uuid::Uuid;
use crate::error::ApiError;

pub async fn create_order(
    db: &DatabaseConnection,
    user_id: Uuid,
) -> Result<orders::Model, ApiError> {
    // ดึงรายการสินค้าจากตะกร้า
    let cart_items = cart::Entity::find()
        .filter(cart::Column::UserId.eq(user_id))
        .all(db)
        .await
        .map_err(ApiError::from)?;

    if cart_items.is_empty() {
        return Err(ApiError::ValidationError(
            "Cart is empty, cannot create order".to_string(),
        ));
    }

    // คำนวณราคารวม
    let mut total_price = Decimal::new(0, 0);
    for item in &cart_items {
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

    // สร้างคำสั่งซื้อใหม่
    let new_order = orders::ActiveModel {
        id: Set(Uuid::new_v4()),
        user_id: Set(user_id),
        total_price: Set(total_price),
        status: Set("pending".to_string()),
        created_at: Set(chrono::Utc::now()),
    };
    let order = new_order.insert(db).await.map_err(ApiError::from)?;

    // เพิ่มสินค้าใน OrderItems
    for item in cart_items {
        let order_item = order_items::ActiveModel {
            id: Set(Uuid::new_v4()),
            order_id: Set(order.id),
            product_id: Set(item.product_id),
            quantity: Set(item.quantity),
            price: Set(item.quantity.into()),
        };
        order_item.insert(db).await.map_err(ApiError::from)?;
    }

    // ลบสินค้าทั้งหมดจากตะกร้า
    cart::Entity::delete_many()
        .filter(cart::Column::UserId.eq(user_id))
        .exec(db)
        .await
        .map_err(ApiError::from)?;

    Ok(order)
}


pub async fn get_order_details(
    db: &DatabaseConnection,
    order_id: Uuid,
) -> Result<(orders::Model, Vec<order_items::Model>), ApiError> {
    let order = orders::Entity::find_by_id(order_id)
        .one(db)
        .await
        .map_err(ApiError::from)?
        .ok_or(ApiError::NotFound(format!("Order with ID {} not found", order_id)))?;

    let order_items = order_items::Entity::find()
        .filter(order_items::Column::OrderId.eq(order_id))
        .all(db)
        .await
        .map_err(ApiError::from)?;

    Ok((order, order_items))
}


pub async fn get_order_history(
    db: &DatabaseConnection,
    user_id: Uuid,
) -> Result<Vec<orders::Model>, ApiError> {
    let orders = orders::Entity::find()
        .filter(orders::Column::UserId.eq(user_id))
        .all(db)
        .await
        .map_err(ApiError::from)?;

    Ok(orders)
}

pub async fn update_order_status(
    db: &DatabaseConnection,
    order_id: Uuid,
    new_status: String,
) -> Result<(), ApiError> {
    let order = orders::Entity::find_by_id(order_id)
        .one(db)
        .await
        .map_err(ApiError::from)?
        .ok_or(ApiError::NotFound(format!("Order with ID {} not found", order_id)))?;

    let mut active_order: orders::ActiveModel = order.into();
    active_order.status = Set(new_status);

    active_order.update(db).await.map_err(ApiError::from)?;
    Ok(())
}
