use super::m20241201_142426_create_table_category::Category;
use super::m20241201_142430_create_table_product::Product;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ProductCategory::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ProductCategory::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(PgFunc::gen_random_uuid()),
                    )
                    .col(ColumnDef::new(ProductCategory::ProductId).uuid().not_null())
                    .col(
                        ColumnDef::new(ProductCategory::CategoryId)
                            .uuid()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(ProductCategory::Table, ProductCategory::ProductId)
                            .to(Product::Table, Product::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(ProductCategory::Table, ProductCategory::CategoryId)
                            .to(Category::Table, Category::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ProductCategory::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum ProductCategory {
    Table,
    Id,
    ProductId,
    CategoryId,
}
