mod test_helpers;
use sea_orm::{ActiveModelTrait, Set};
use test_helpers::{new_test_db, cleanup_test_dbs};

// Define the Region entity using sea-orm macros
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "region")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub slug: String,
    pub katakana: String,
    pub english: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

#[tokio::test]
async fn test_region_basic_query() {
    // Setup test database
    let db = new_test_db().await;
    
    // Create a region using ActiveModel
    let region = ActiveModel {
        // Use NotSet for id to let the database assign it
        id: sea_orm::ActiveValue::NotSet,
        slug: Set("tokyo".to_string()),
        katakana: Set("トウキョウ".to_string()),
        english: Set("Tokyo".to_string()),
    };
    
    // Insert the region into the database
    let inserted_region = region.insert(&db).await.expect("Failed to insert region");
    
    // Verify the region was inserted correctly
    assert_eq!(inserted_region.slug, "tokyo");
    assert_eq!(inserted_region.katakana, "トウキョウ");
    assert_eq!(inserted_region.english, "Tokyo");
    
    // For SQLite, the first ID should be 1
    assert_eq!(inserted_region.id, 1);
    
    // Clean up - explicitly drop the database connection
    drop(db);
    cleanup_test_dbs();
}

#[tokio::test]
async fn test_find_region_by_slug() {
    // Setup test database
    let db = new_test_db().await;
    
    // Create and insert two regions
    let _tokyo = ActiveModel {
        id: sea_orm::ActiveValue::NotSet,
        slug: Set("tokyo".to_string()),
        katakana: Set("トウキョウ".to_string()),
        english: Set("Tokyo".to_string()),
    }.insert(&db).await.expect("Failed to insert Tokyo");
    
    let osaka = ActiveModel {
        id: sea_orm::ActiveValue::NotSet,
        slug: Set("osaka".to_string()),
        katakana: Set("オオサカ".to_string()),
        english: Set("Osaka".to_string()),
    }.insert(&db).await.expect("Failed to insert Osaka");
    
    // Use a custom query to find a region by its slug
    use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
    
    // Find region by slug
    let found_region = Entity::find()
        .filter(Column::Slug.eq("osaka"))
        .one(&db)
        .await
        .expect("Failed to execute query");
    
    // Verify we found the right region
    match found_region {
        Some(region) => {
            assert_eq!(region.id, osaka.id);
            assert_eq!(region.slug, "osaka");
            assert_eq!(region.katakana, "オオサカ");
            assert_eq!(region.english, "Osaka");
        },
        None => panic!("Region not found"),
    }
    
    // Clean up - explicitly drop the database connection
    drop(db);
    cleanup_test_dbs();
}