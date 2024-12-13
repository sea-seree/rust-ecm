use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create Users Table
        manager
            .create_table(
                Table::create()
                    .table(Users::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Users::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Users::Username).string().not_null().unique_key())
                    .col(ColumnDef::new(Users::Email).string().not_null().unique_key())
                    .col(ColumnDef::new(Users::HashedPassword).string().not_null())
                    .col(ColumnDef::new(Users::CreatedAt).timestamp_with_time_zone().not_null())
                    .to_owned(),
            )
            .await?;

        // Create Products Table
        manager
            .create_table(
                Table::create()
                    .table(Products::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Products::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Products::Name).string().not_null())
                    .col(ColumnDef::new(Products::Description).string())
                    .col(ColumnDef::new(Products::Price).decimal().not_null())
                    .col(ColumnDef::new(Products::Status).string().not_null().default("available")) // เพิ่มคอลัมน์ status
                    .col(ColumnDef::new(Products::CreatedAt).timestamp_with_time_zone().not_null())
                    .to_owned(),
            )
            .await?;

        // Create Cart Table
        manager
            .create_table(
                Table::create()
                    .table(Cart::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Cart::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Cart::UserId).uuid().not_null())
                    .col(ColumnDef::new(Cart::ProductId).uuid().not_null())
                    .col(ColumnDef::new(Cart::Quantity).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(Cart::Table, Cart::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Cart::Table, Cart::ProductId)
                            .to(Products::Table, Products::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

            manager
            .create_table(
                Table::create()
                    .table(Orders::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Orders::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Orders::UserId).uuid().not_null())
                    .col(ColumnDef::new(Orders::TotalPrice).decimal().not_null())
                    .col(ColumnDef::new(Orders::Status).string().not_null().default("pending"))
                    .col(ColumnDef::new(Orders::CreatedAt).timestamp_with_time_zone().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(Orders::Table, Orders::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Create OrderItems Table
        manager
            .create_table(
                Table::create()
                    .table(OrderItems::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(OrderItems::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(OrderItems::OrderId).uuid().not_null())
                    .col(ColumnDef::new(OrderItems::ProductId).uuid().not_null())
                    .col(ColumnDef::new(OrderItems::Quantity).integer().not_null())
                    .col(ColumnDef::new(OrderItems::Price).decimal().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(OrderItems::Table, OrderItems::OrderId)
                            .to(Orders::Table, Orders::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(OrderItems::Table, OrderItems::ProductId)
                            .to(Products::Table, Products::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(OrderItems::Table).to_owned()).await?;
        // Drop Orders Table
        manager.drop_table(Table::drop().table(Orders::Table).to_owned()).await?;
        // Drop Cart Table
        manager.drop_table(Table::drop().table(Cart::Table).to_owned()).await?;
        // Drop Products Table
        manager.drop_table(Table::drop().table(Products::Table).to_owned()).await?;
        // Drop Users Table
        manager.drop_table(Table::drop().table(Users::Table).to_owned()).await?;
        Ok(())
    }
}

#[derive(Iden)]
pub enum Users {
    Table,
    Id,
    Username,
    Email,
    HashedPassword,
    CreatedAt,
}

#[derive(Iden)]
pub enum Products {
    Table,
    Id,
    Name,
    Description,
    Price,
    Status,        // เพิ่มใน Enum
    CreatedAt,
}

#[derive(Iden)]
pub enum Cart {
    Table,
    Id,
    UserId,
    ProductId,
    Quantity,
}

#[derive(Iden)]
pub enum Orders {
    Table,
    Id,
    UserId,
    TotalPrice,
    Status,
    CreatedAt,
}

#[derive(Iden)]
pub enum OrderItems {
    Table,
    Id,
    OrderId,
    ProductId,
    Quantity,
    Price,
}
