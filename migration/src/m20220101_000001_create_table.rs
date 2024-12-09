use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Users::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Users::Id)
                        .uuid()
                        .not_null()
                        .primary_key())
                    .col(ColumnDef::new(Users::Username)
                        .string()
                        .not_null()
                        .unique_key())
                    .col(ColumnDef::new(Users::Email)
                        .string()
                        .not_null()
                        .unique_key())
                    .col(ColumnDef::new(Users::HashedPassword)
                        .string()
                        .not_null())
                    .col(ColumnDef::new(Users::CreatedAt)
                        .timestamp_with_time_zone()
                        .not_null()
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(Users::Table).to_owned()).await
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
