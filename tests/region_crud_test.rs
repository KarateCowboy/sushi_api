mod test_helpers;
use sea_orm::{ActiveModelTrait, Set};
use test_helpers::{new_test_db, cleanup_test_dbs};

use sushi_api::models::region::{ActiveModel, Entity, Column};
use spectral::prelude::*;

#[tokio::test]
async fn test_region_basic_query() {
    let db = new_test_db().await;
    
    let region = ActiveModel {
        id: sea_orm::ActiveValue::NotSet,
        slug: Set("tokyo".to_string()),
        katakana: Set("トウキョウ".to_string()),
        english: Set("Tokyo".to_string()),
    };
    
    let inserted_region = region.insert(&db).await.expect("Failed to insert region");
    
    asserting!("inserted region")
        .that(&inserted_region.slug)
        .is_equal_to("tokyo".to_string());
   
    asserting!("inserted region")
        .that(&inserted_region.katakana)
        .is_equal_to("トウキョウ".to_string());
    
    asserting!("inserted region")
        .that(&inserted_region.english)
        .is_equal_to("Tokyo".to_string());
    
    asserting!("inserted region")
        .that(&inserted_region.id)
        .is_greater_than(0);
    
    db.close().await.expect("Failed to close database connection");
    cleanup_test_dbs().await;
}

#[tokio::test]
async fn test_find_region_by_slug() {
    let db = new_test_db().await;
    
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
    
    use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
    
    let found_region = Entity::find()
        .filter(Column::Slug.eq("osaka"))
        .one(&db)
        .await
        .expect("Failed to execute query");
    
    asserting!("found region")
        .that(&found_region)
        .is_some();
    
    if let Some(region) = found_region {
        asserting!("found region")
            .that(&region.id)
            .is_equal_to(&osaka.id);
            
        asserting!("found region")
            .that(&region.slug)
            .is_equal_to(&"osaka".to_string());
            
        asserting!("found region")
            .that(&region.katakana)
            .is_equal_to(&"オオサカ".to_string());
            
        asserting!("found region")
            .that(&region.english)
            .is_equal_to(&"Osaka".to_string());
    }
    
    db.close().await.expect("Failed to close database connection");
    cleanup_test_dbs().await;
}
