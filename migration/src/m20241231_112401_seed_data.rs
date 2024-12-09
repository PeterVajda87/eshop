use entity::*;
use sea_orm_migration::prelude::*;
use sea_orm_migration::sea_orm::entity::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        let language = language::ActiveModel {
            code: Set("en".to_owned()),
            title: Set("English".to_owned()),
            ..Default::default()
        }
        .insert(db)
        .await?;

        let language_id = language.id;

        user::ActiveModel {
            email: Set("peter.vajda@post.cz".to_owned()),
            ..Default::default()
        }
        .insert(db)
        .await?;

        let category = category::ActiveModel {
            ..Default::default()
        }
        .insert(db)
        .await?;

        let product = product::ActiveModel {
            ..Default::default()
        }
        .insert(db)
        .await?;

        let category_id = category.id;
        let product_id = product.id;

        product_category::ActiveModel {
            product_id: Set(product_id),
            category_id: Set(category_id),
            ..Default::default()
        }
        .insert(db)
        .await?;

        product_translation::ActiveModel {
            product_id: Set(product_id),
            language_id: Set(language_id),
            title: Set(Some("Skoda Felicia".to_owned())),
            description: Set(Some("Cool car for family with kids and budget".to_owned())),
            ..Default::default()
        }
        .insert(db)
        .await?;

        category_translation::ActiveModel {
            category_id: Set(category_id),
            language_id: Set(language_id),
            title: Set(Some("Skoda family cars".to_owned())),
            description: Set(Some(
                "This category contains multiple models of Skoda family cars".to_owned(),
            )),
            ..Default::default()
        }
        .insert(db)
        .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        product_category::Entity::delete_many().exec(db).await?;
        product::Entity::delete_many().exec(db).await?;
        category::Entity::delete_many().exec(db).await?;
        user::Entity::delete_many().exec(db).await?;

        Ok(())
    }
}
