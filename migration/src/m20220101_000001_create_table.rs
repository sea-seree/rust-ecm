use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(User::Id)
                        .uuid()
                        .not_null()
                        .primary_key())
                    .col(ColumnDef::new(User::Username)
                        .string()
                        .not_null()
                        .unique_key())
                    .col(ColumnDef::new(User::Email)
                        .string()
                        .not_null()
                        .unique_key())
                    .col(ColumnDef::new(User::HashedPassword)
                        .string()
                        .not_null())
                    .col(ColumnDef::new(User::CreatedAt)
                        .date_time()
                        .not_null()
                        .default("now()".to_string())
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(User::Table).to_owned()).await
    }
}

#[derive(Iden)]
pub enum User {
    Table,
    Id,
    Username,
    Email,
    HashedPassword,
    CreatedAt,
}
