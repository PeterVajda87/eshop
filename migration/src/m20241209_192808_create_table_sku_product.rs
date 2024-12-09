use super::m20241201_142430_create_table_product::Product;
use super::m20241209_192555_create_table_sku::Sku;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(SkuProduct::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(SkuProduct::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(PgFunc::gen_random_uuid()),
                    )
                    .col(ColumnDef::new(SkuProduct::ProductId).uuid().not_null())
                    .col(ColumnDef::new(SkuProduct::SkuId).uuid().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(SkuProduct::Table, SkuProduct::ProductId)
                            .to(Product::Table, Product::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(SkuProduct::Table, SkuProduct::SkuId)
                            .to(Sku::Table, Sku::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(SkuProduct::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum SkuProduct {
    Table,
    Id,
    SkuId,
    ProductId,
}
