use sea_orm_migration::prelude::*;
use sea_orm::Statement;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Get reference to database connection
        let db = manager.get_connection();
        
        // Insert region data using raw SQL for better control
        db.execute(
            Statement::from_string(
                db.get_database_backend(),
                r#"
                INSERT INTO region (slug, katakana, english)
                VALUES
                ('chicago', 'シカゴ', 'Chicago'),
                ('columbus', 'コロンバス', 'Columbus'),
                ('san-diego', 'サンディエゴ', 'San Diego'),
                ('los-angeles', 'ロサンゼルス', 'Los Angeles'),
                ('new-york', 'ニューヨーク', 'New York'),
                ('boston', 'ボストン', 'Boston'),
                ('nashville', 'ナッシュビル', 'Nashville');
                "#
            )
        ).await?;
        
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Delete all seeded regions
        manager
            .get_connection()
            .execute(
                Statement::from_string(
                    manager.get_database_backend(),
                    "DELETE FROM region"
                )
            )
            .await?;
            
        Ok(())
    }
}
