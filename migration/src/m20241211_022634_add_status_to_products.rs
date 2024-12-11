use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
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
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(Products::Table).to_owned()).await
    }
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
