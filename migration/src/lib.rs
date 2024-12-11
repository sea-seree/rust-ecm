pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_table;
mod m20241209_064312_products;
mod m20241211_022634_add_status_to_products;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_table::Migration),
            Box::new(m20241209_064312_products::Migration),
            Box::new(m20241211_022634_add_status_to_products::Migration),
        ]
    }
}
