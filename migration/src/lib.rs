pub use sea_orm_migration::prelude::*;

mod m20241130_132845_create_table_user;
mod m20241130_160519_create_table_language;
mod m20241201_142426_create_table_category;
mod m20241201_142430_create_table_product;
mod m20241206_175614_create_table_product_category;
mod m20241231_112401_seed_data;
mod m20241209_192555_create_table_sku;
mod m20241209_192808_create_table_sku_product;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20241130_132845_create_table_user::Migration),
            Box::new(m20241130_160519_create_table_language::Migration),
            Box::new(m20241201_142426_create_table_category::Migration),
            Box::new(m20241201_142430_create_table_product::Migration),
            Box::new(m20241206_175614_create_table_product_category::Migration),
            Box::new(m20241231_112401_seed_data::Migration),
            Box::new(m20241209_192555_create_table_sku::Migration),
            Box::new(m20241209_192808_create_table_sku_product::Migration),
        ]
    }
}
