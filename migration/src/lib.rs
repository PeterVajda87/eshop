pub use sea_orm_migration::prelude::*;

mod m20241130_132845_create_table_user;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m20241130_132845_create_table_user::Migration)]
    }
}
