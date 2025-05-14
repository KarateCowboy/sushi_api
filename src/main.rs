use dotenv::dotenv;
use poem::{Server, Route, EndpointExt, middleware::Cors, get, handler};
use sea_orm::{Database, DatabaseConnection};
use std::env;

pub mod models;
pub mod api;

#[handler]
fn health() -> &'static str {
    "OK"
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // Load environment variables
    dotenv().ok();

    // Configure database connection
    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "sqlite:sushi.db?mode=rwc".to_string());

    let db: DatabaseConnection = Database::connect(&database_url)
        .await
        .expect("Failed to connect to database");

    // Run migrations if needed (in development)
    if env::var("RUN_MIGRATIONS").unwrap_or_else(|_| "true".to_string()) == "true" {
        migration::Migrator::up(&db, None)
            .await
            .expect("Migration failed");
    }

    // Define server host and port
    let host = env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = env::var("PORT")
        .unwrap_or_else(|_| "8000".to_string())
        .parse::<u16>()
        .expect("Invalid PORT");

    println!("Starting server at http://{}:{}", host, port);

    // Create API routes
    let app = Route::new()
        .nest("/api", api::create_routes(db.clone()))
        .at("/health", get(health))
        .with(Cors::new());

    // Start the server
    Server::new(poem::net::TcpListener::bind(format!("{}:{}", host, port)))
        .run(app)
        .await
}