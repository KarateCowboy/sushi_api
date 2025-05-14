pub mod regions;

use poem::{Route, endpoint::StaticFilesEndpoint};
use sea_orm::DatabaseConnection;

pub fn create_routes(db: DatabaseConnection) -> Route {
    Route::new()
        .nest("/regions", regions::routes(db.clone()))
        .nest("/static", StaticFilesEndpoint::new("static"))
}