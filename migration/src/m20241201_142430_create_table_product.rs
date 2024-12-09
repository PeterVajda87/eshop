use crate::m20241130_160519_create_table_language::Language;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Product::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Product::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(PgFunc::gen_random_uuid()),
                    )
                    .to_owned(),
            )
            .await
            .unwrap();

        manager
            .create_table(
                Table::create()
                    .table(ProductTranslation::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ProductTranslation::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(PgFunc::gen_random_uuid()),
                    )
                    .col(
                        ColumnDef::new(ProductTranslation::LanguageId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ProductTranslation::ProductId)
                            .uuid()
                            .not_null(),
                    )
                    .col(ColumnDef::new(ProductTranslation::Title).text())
                    .col(ColumnDef::new(ProductTranslation::Description).text())
                    .foreign_key(
                        ForeignKey::create()
                            .from(ProductTranslation::Table, ProductTranslation::LanguageId)
                            .to(Language::Table, Language::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(ProductTranslation::Table, ProductTranslation::ProductId)
                            .to(Product::Table, Product::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Product::Table).to_owned())
            .await
            .unwrap();

        manager
            .drop_table(Table::drop().table(ProductTranslation::Table).to_owned())
            .await
            .unwrap();

        Ok(())
    }
}

#[derive(DeriveIden)]
pub enum Product {
    Table,
    Id,
}

#[derive(DeriveIden)]
pub enum ProductTranslation {
    Table,
    Id,
    LanguageId,
    ProductId,
    Title,
    Description,
}
