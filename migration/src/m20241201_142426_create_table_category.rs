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
                    .table(Category::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Category::Id)
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
                    .table(CategoryTranslation::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(CategoryTranslation::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(PgFunc::gen_random_uuid()),
                    )
                    .col(
                        ColumnDef::new(CategoryTranslation::LanguageId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(CategoryTranslation::CategoryId)
                            .uuid()
                            .not_null(),
                    )
                    .col(ColumnDef::new(CategoryTranslation::Title).text())
                    .col(ColumnDef::new(CategoryTranslation::Description).text())
                    .foreign_key(
                        ForeignKey::create()
                            .from(CategoryTranslation::Table, CategoryTranslation::LanguageId)
                            .to(Language::Table, Language::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(CategoryTranslation::Table, CategoryTranslation::CategoryId)
                            .to(Category::Table, Category::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Category::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Category {
    Table,
    Id,
}

#[derive(DeriveIden)]
pub enum CategoryTranslation {
    Table,
    Id,
    LanguageId,
    CategoryId,
    Title,
    Description,
}
