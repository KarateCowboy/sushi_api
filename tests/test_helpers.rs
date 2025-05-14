use sea_orm::{Database, DatabaseConnection, ConnectOptions};
use sea_orm_migration::MigratorTrait;
use std::path::Path;
use tokio::fs;
use std::time::Duration;
use uuid::Uuid;

pub async fn new_test_db() -> DatabaseConnection {
    let unique_id = Uuid::new_v4().to_string();
    let filename = format!("test_sushi_{}.db", unique_id);
    let db_path = Path::new(&filename);
    
    if db_path.exists() {
        let _ = std::fs::remove_file(db_path);
    }
    
    let db_url = format!("sqlite:{}?mode=rwc", db_path.display());
    
    let mut opt = ConnectOptions::new(db_url);
    opt.max_connections(5)
        .min_connections(1)
        .connect_timeout(Duration::from_secs(3))
        .idle_timeout(Duration::from_secs(3))
        .max_lifetime(Duration::from_secs(5));
    
    let db = Database::connect(opt)
        .await
        .expect("Failed to connect to test database");
    
    migration::Migrator::up(&db, None)
        .await
        .expect("Failed to run migrations");
    db
}

pub async fn cleanup_test_dbs() {
    tokio::time::sleep(Duration::from_millis(50)).await;
    
    if let Ok(mut entries) = fs::read_dir(".").await {
        while let Ok(Some(entry)) = entries.next_entry().await {
            if let Ok(file_name) = entry.file_name().into_string() {
                if file_name.starts_with("test_sushi_") && file_name.ends_with(".db") {
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


