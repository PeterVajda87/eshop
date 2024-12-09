//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.1

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "product")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::product_category::Entity")]
    ProductCategory,
    #[sea_orm(has_many = "super::product_translation::Entity")]
    ProductTranslation,
    #[sea_orm(has_many = "super::sku_product::Entity")]
    SkuProduct,
}

impl Related<super::product_category::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ProductCategory.def()
    }
}

impl Related<super::product_translation::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ProductTranslation.def()
    }
}

impl Related<super::sku_product::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::SkuProduct.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}