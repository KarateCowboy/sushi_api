mod test_helpers;
mod api_test_helpers;

use sea_orm::{EntityTrait, ColumnTrait, QueryFilter, DatabaseConnection, Set, ActiveModelTrait};
use spectral::prelude::*;
use test_helpers::{new_test_db, cleanup_test_dbs};
use sushi_api::models::region::{self, Entity as Region};

#[tokio::test]
async fn test_can_retrieve_all_regions() {
    // Setup test database
    let db = new_test_db().await;
    
    // Seed with test data
    api_test_helpers::seed_test_regions(&db).await;
    
    // Test retrieving all regions
    let regions = Region::find()
        .all(&db)
        .await
        .expect("Failed to retrieve regions");
    
    // Assertions
    asserting!("all regions are retrieved")
        .that(&regions.len())
        .is_equal_to(7);
    
    // Check if specific regions exist
    let chicago = regions.iter().find(|r| r.slug == "chicago");
    asserting!("chicago region exists")
        .that(&chicago)
        .is_some();
    
    asserting!("chicago has correct katakana")
        .that(&chicago.unwrap().katakana)
        .is_equal_to("シカゴ".to_string());
    
    // Cleanup
    db.close().await.expect("Failed to close database connection");
    cleanup_test_dbs().await;
}

#[tokio::test]
async fn test_can_retrieve_region_by_slug() {
    // Setup test database
    let db = new_test_db().await;
    
    // Create a specific test region
    let test_region = api_test_helpers::create_test_region(
        &db, 
        "boston", 
        "ボストン", 
        "Boston"
    ).await;
    
    // Test retrieving region by slug
    let region = Region::find()
        .filter(region::Column::Slug.eq("boston"))
        .one(&db)
        .await
        .expect("Failed to run query")
        .expect("Region should exist");
    
    // Assertions
    asserting!("region id matches")
        .that(&region.id)
        .is_equal_to(test_region.id);
    
    asserting!("region slug matches")
        .that(&region.slug)
        .is_equal_to("boston".to_string());
    
    asserting!("region katakana matches")
        .that(&region.katakana)
        .is_equal_to("ボストン".to_string());
    
    asserting!("region english matches")
        .that(&region.english)
        .is_equal_to("Boston".to_string());
    
    // Cleanup
    db.close().await.expect("Failed to close database connection");
    cleanup_test_dbs().await;
}

#[tokio::test]
async fn test_can_filter_regions_by_name() {
    // Setup test database
    let db = new_test_db().await;
    
    // Seed with test data
    api_test_helpers::seed_test_regions(&db).await;
    
    // Test filtering regions with "n" in the slug
    let regions = Region::find()
        .filter(region::Column::Slug.contains("n"))
        .all(&db)
        .await
        .expect("Failed to retrieve filtered regions");
    
    // Should include: san-diego, nashville, new-york, los-angeles
    asserting!("filtered regions count")
        .that(&regions.len())
        .is_greater_than_or_equal_to(4);
    
    // Check for specific cities
    let has_nashville = regions.iter().any(|r| r.slug == "nashville");
    let has_new_york = regions.iter().any(|r| r.slug == "new-york");
    let has_san_diego = regions.iter().any(|r| r.slug == "san-diego");
    
    asserting!("nashville is in filtered results")
        .that(&has_nashville)
        .is_true();
    
    asserting!("new-york is in filtered results")
        .that(&has_new_york)
        .is_true();
    
    asserting!("san-diego is in filtered results")
        .that(&has_san_diego)
        .is_true();
    
    // Cleanup
    db.close().await.expect("Failed to close database connection");
    cleanup_test_dbs().await;
}

#[tokio::test]
async fn test_region_not_found_returns_none() {
    // Setup test database
    let db = new_test_db().await;
    
    // Seed with test data
    api_test_helpers::seed_test_regions(&db).await;
    
    // Test retrieving a non-existent region
    let region = Region::find()
        .filter(region::Column::Slug.eq("nonexistent"))
        .one(&db)
        .await
        .expect("Failed to run query");
    
    // Assertions
    asserting!("nonexistent region returns None")
        .that(&region)
        .is_none();
    
    // Cleanup
    db.close().await.expect("Failed to close database connection");
    cleanup_test_dbs().await;
}

#[tokio::test]
async fn test_can_create_new_region() {
    // Setup test database
    let db = new_test_db().await;
    
    // Create a new region
    let seattle = api_test_helpers::create_test_region(
        &db,
        "seattle",
        "シアトル",
        "Seattle"
    ).await;
    
    // Verify it was created correctly
    asserting!("seattle was created with correct ID")
        .that(&seattle.id)
        .is_greater_than(0);
    
    asserting!("seattle slug")
        .that(&seattle.slug)
        .is_equal_to("seattle".to_string());
    
    asserting!("seattle katakana")
        .that(&seattle.katakana)
        .is_equal_to("シアトル".to_string());
    
    asserting!("seattle english")
        .that(&seattle.english)
        .is_equal_to("Seattle".to_string());
    
    // Retrieve it again to verify it persisted
    let retrieved = Region::find()
        .filter(region::Column::Slug.eq("seattle"))
        .one(&db)
        .await
        .expect("Failed to run query")
        .expect("Seattle should exist");
    
    asserting!("retrieved seattle matches")
        .that(&retrieved.id)
        .is_equal_to(seattle.id);
    
    // Cleanup
    db.close().await.expect("Failed to close database connection");
    cleanup_test_dbs().await;
}

#[tokio::test]
async fn test_can_update_existing_region() {
    // Setup test database
    let db = new_test_db().await;
    
    // Create a region to update
    let original = api_test_helpers::create_test_region(
        &db,
        "boston",
        "ボストン",
        "Boston"
    ).await;
    
    // Update the region
    let mut boston_model: region::ActiveModel = original.clone().into();
    boston_model.katakana = Set("ボストンシ".to_string());
    boston_model.english = Set("Boston City".to_string());
    
    let updated = boston_model.update(&db)
        .await
        .expect("Failed to update region");
    
    // Assertions
    asserting!("updated ID matches original")
        .that(&updated.id)
        .is_equal_to(original.id);
    
    asserting!("slug remains unchanged")
        .that(&updated.slug)
        .is_equal_to("boston".to_string());
    
    asserting!("katakana was updated")
        .that(&updated.katakana)
        .is_equal_to("ボストンシ".to_string());
    
    asserting!("english was updated")
        .that(&updated.english)
        .is_equal_to("Boston City".to_string());
    
    // Verify updated record persisted
    let retrieved = Region::find()
        .filter(region::Column::Slug.eq("boston"))
        .one(&db)
        .await
        .expect("Failed to run query")
        .expect("Boston should exist");
    
    asserting!("retrieved record has updated katakana")
        .that(&retrieved.katakana)
        .is_equal_to("ボストンシ".to_string());
    
    // Cleanup
    db.close().await.expect("Failed to close database connection");
    cleanup_test_dbs().await;
}

#[tokio::test]
async fn test_can_delete_region() {
    // Setup test database
    let db = new_test_db().await;
    
    // Create a region to delete
    let nashville = api_test_helpers::create_test_region(
        &db,
        "nashville",
        "ナッシュビル",
        "Nashville"
    ).await;
    
    // Verify it exists
    let exists_before = Region::find()
        .filter(region::Column::Id.eq(nashville.id))
        .one(&db)
        .await
        .expect("Failed to run query")
        .is_some();
    
    asserting!("nashville exists before deletion")
        .that(&exists_before)
        .is_true();
    
    // Delete the region
    let nashville_model: region::ActiveModel = nashville.into();
    let delete_result = nashville_model.delete(&db)
        .await
        .expect("Failed to delete region");
    
    asserting!("deletion was successful")
        .that(&delete_result.rows_affected)
        .is_greater_than(0);
    
    // Verify it no longer exists
    let exists_after = Region::find()
        .filter(region::Column::Slug.eq("nashville"))
        .one(&db)
        .await
        .expect("Failed to run query")
        .is_none();
    
    asserting!("nashville doesn't exist after deletion")
        .that(&exists_after)
        .is_true();
    
    // Cleanup
    db.close().await.expect("Failed to close database connection");
    cleanup_test_dbs().await;
}