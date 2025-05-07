use sea_orm::{Database, DatabaseConnection};
use sea_orm_migration::{MigratorTrait, SchemaManager};
use std::path::Path;

/// Creates a test SQLite database file if it does not exist.
/// Applies migrations to ensure the database schema is up-to-date.
/// Returns the database connection pool.
pub async fn new_test_db() -> DatabaseConnection {
    // Define the test database file path
    let db_path = Path::new("test_sushi.db");
    let db_url = format!("sqlite:{}?mode=rwc", db_path.display());
    
    // Connect to the database (will create it if it doesn't exist)
    let db = Database::connect(&db_url)
        .await
        .expect("Failed to connect to test database");
    
    // Apply migrations from the migration workspace
    
    // Use the Migrator from the migration crate
    migration::Migrator::up(&db, None)
        .await
        .expect("Failed to run migrations");
    
    // Return the database connection
    db
}