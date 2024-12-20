//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.1

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "product_translation")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub language_id: Uuid,
    pub product_id: Uuid,
    #[sea_orm(column_type = "Text", nullable)]
    pub title: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub description: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::language::Entity",
        from = "Column::LanguageId",
        to = "super::language::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Language,
    #[sea_orm(
        belongs_to = "super::product::Entity",
        from = "Column::ProductId",
        to = "super::product::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Product,
}

impl Related<super::language::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Language.def()
    }
}

impl Related<super::product::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Product.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
