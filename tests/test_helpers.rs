use sea_orm::{Database, DatabaseConnection, ConnectOptions};
use sea_orm_migration::MigratorTrait;
use std::path::Path;
use tokio::fs;
use std::time::Duration;
use uuid::Uuid;

/// Creates a fresh test SQLite database file for each test run.
/// Uses a unique filename to avoid conflicts between tests.
/// Applies migrations to ensure the database schema is up-to-date.
/// Returns the database connection pool.
pub async fn new_test_db() -> DatabaseConnection {
    // Generate a unique filename for each test run
    let unique_id = Uuid::new_v4().to_string();
    let filename = format!("test_sushi_{}.db", unique_id);
    let db_path = Path::new(&filename);
    
    // Ensure any existing file with the same name is removed
    if db_path.exists() {
        let _ = std::fs::remove_file(db_path);
    }
    
    // Create a clean database URL with explicit mode for read-write with create
    let db_url = format!("sqlite:{}?mode=rwc", db_path.display());
    
    // Configure connection options with shorter timeouts
    let mut opt = ConnectOptions::new(db_url);
    opt.max_connections(5)
        .min_connections(1)
        .connect_timeout(Duration::from_secs(3))
        .idle_timeout(Duration::from_secs(3))
        .max_lifetime(Duration::from_secs(5));
    
    // Connect to the database (will create it if it doesn't exist)
    let db = Database::connect(opt)
        .await
        .expect("Failed to connect to test database");
    
    // Apply migrations from the migration workspace
    migration::Migrator::up(&db, None)
        .await
        .expect("Failed to run migrations");
    
    // Return the database connection
    db
}

/// Cleanup function to remove test database files
/// Call this at the end of your test suite
pub async fn cleanup_test_dbs() {
    // Small delay to ensure database connections are properly closed
    tokio::time::sleep(Duration::from_millis(50)).await;
    
    // Find and remove test database files
    if let Ok(mut entries) = fs::read_dir(".").await {
        while let Ok(Some(entry)) = entries.next_entry().await {
            if let Ok(file_name) = entry.file_name().into_string() {
                if file_name.starts_with("test_sushi_") && file_name.ends_with(".db") {
                    // Try multiple times in case of file locking issues
                    for _ in 0..3 {
                        if fs::remove_file(&entry.path()).await.is_ok() {
                            break;
                        }
                        tokio::time::sleep(Duration::from_millis(50)).await;
                    }
                }
            }
        }
    }
}