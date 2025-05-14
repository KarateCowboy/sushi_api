use sea_orm::{DatabaseConnection, Set, ActiveValue::NotSet};
use sushi_api::models::region::{self, ActiveModel as RegionModel};

// Function to create test regions in the database
pub async fn seed_test_regions(db: &DatabaseConnection) {
    let regions = vec![
        ("chicago", "シカゴ", "Chicago"),
        ("columbus", "コロンバス", "Columbus"),
        ("san-diego", "サンディエゴ", "San Diego"),
        ("los-angeles", "ロサンゼルス", "Los Angeles"),
        ("new-york", "ニューヨーク", "New York"),
        ("boston", "ボストン", "Boston"),
        ("nashville", "ナッシュビル", "Nashville"),
    ];
    
    for (slug, katakana, english) in regions {
        let region = RegionModel {
            id: NotSet,
            slug: Set(slug.to_string()),
            katakana: Set(katakana.to_string()),
            english: Set(english.to_string()),
        };
        
        region.insert(db).await.expect("Failed to insert region");
    }
}

// Function to create a specific test region
pub async fn create_test_region(
    db: &DatabaseConnection,
    slug: &str,
    katakana: &str,
    english: &str
) -> region::Model {
    let region = RegionModel {
        id: NotSet,
        slug: Set(slug.to_string()),
        katakana: Set(katakana.to_string()),
        english: Set(english.to_string()),
    };
    
    region.insert(db).await.expect("Failed to insert region")
}

// Add more helper functions as needed