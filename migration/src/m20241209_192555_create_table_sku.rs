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
                    .table(Sku::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Sku::Id)
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
                    .table(SkuTranslation::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(SkuTranslation::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(PgFunc::gen_random_uuid()),
                    )
                    .col(ColumnDef::new(SkuTranslation::LanguageId).uuid().not_null())
                    .col(ColumnDef::new(SkuTranslation::SkuId).uuid().not_null())
                    .col(ColumnDef::new(SkuTranslation::Title).text())
                    .col(ColumnDef::new(SkuTranslation::Description).text())
                    .foreign_key(
                        ForeignKey::create()
                            .from(SkuTranslation::Table, SkuTranslation::LanguageId)
                            .to(Language::Table, Language::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(SkuTranslation::Table, SkuTranslation::SkuId)
                            .to(Sku::Table, Sku::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Sku::Table).to_owned())
            .await
            .unwrap();

        manager
            .drop_table(Table::drop().table(SkuTranslation::Table).to_owned())
            .await
            .unwrap();

        Ok(())
    }
}

#[derive(DeriveIden)]
pub enum Sku {
    Table,
    Id,
}

#[derive(DeriveIden)]
pub enum SkuTranslation {
    Table,
    Id,
    LanguageId,
    SkuId,
    Title,
    Description,
}
