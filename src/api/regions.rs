use poem::{
    handler,
    web::{Data, Json, Path, Query},
    Result, Error, http::StatusCode,
    Route, get, post, put, delete,
};
use sea_orm::{
    DatabaseConnection, EntityTrait, Set, ActiveModelTrait,
    QueryFilter, ColumnTrait, ActiveValue::NotSet,
};
use serde::{Deserialize, Serialize};

use crate::models::region::{self, Entity as Region, Model as RegionModel};

// Request and response types
#[derive(Deserialize)]
pub struct CreateRegion {
    pub slug: String,
    pub katakana: String,
    pub english: String,
}

#[derive(Deserialize)]
pub struct UpdateRegion {
    pub katakana: Option<String>,
    pub english: Option<String>,
}

#[derive(Deserialize)]
pub struct FilterQuery {
    pub filter: Option<String>,
}

// Create routes
pub fn routes(db: DatabaseConnection) -> Route {
    let db_data = Data(db);
    
    Route::new()
        .at("/", get(get_regions.data(db_data.clone()))
                 .post(create_region.data(db_data.clone())))
        .at("/:slug", get(get_region_by_slug.data(db_data.clone()))
                      .put(update_region.data(db_data.clone()))
                      .delete(delete_region.data(db_data.clone())))
}

// Handler implementations
#[handler]
async fn get_regions(db: Data<&DatabaseConnection>, query: Query<FilterQuery>) -> Result<Json<Vec<RegionModel>>> {
    let mut query_builder = Region::find();
    
    // Apply filter if provided
    if let Some(filter) = &query.0.filter {
        query_builder = query_builder.filter(
            region::Column::Slug.contains(filter)
                .or(region::Column::English.contains(filter))
                .or(region::Column::Katakana.contains(filter))
        );
    }
    
    let regions = query_builder.all(db.0)
        .await
        .map_err(|e| Error::from_string(e.to_string(), StatusCode::INTERNAL_SERVER_ERROR))?;
    
    Ok(Json(regions))
}

#[handler]
async fn get_region_by_slug(db: Data<&DatabaseConnection>, Path(slug): Path<String>) -> Result<Json<RegionModel>> {
    let region = Region::find()
        .filter(region::Column::Slug.eq(slug))
        .one(db.0)
        .await
        .map_err(|e| Error::from_string(e.to_string(), StatusCode::INTERNAL_SERVER_ERROR))?;
    
    match region {
        Some(region) => Ok(Json(region)),
        None => Err(Error::from_string("Region not found", StatusCode::NOT_FOUND)),
    }
}

#[handler]
async fn create_region(db: Data<&DatabaseConnection>, Json(payload): Json<CreateRegion>) -> Result<Json<RegionModel>> {
    // Check if region with this slug already exists
    let existing = Region::find()
        .filter(region::Column::Slug.eq(&payload.slug))
        .one(db.0)
        .await
        .map_err(|e| Error::from_string(e.to_string(), StatusCode::INTERNAL_SERVER_ERROR))?;
    
    if existing.is_some() {
        return Err(Error::from_string(
            format!("Region with slug '{}' already exists", payload.slug),
            StatusCode::BAD_REQUEST
        ));
    }
    
    // Create new region
    let region = region::ActiveModel {
        id: NotSet,
        slug: Set(payload.slug),
        katakana: Set(payload.katakana),
        english: Set(payload.english),
    };
    
    let result = region.insert(db.0)
        .await
        .map_err(|e| Error::from_string(e.to_string(), StatusCode::INTERNAL_SERVER_ERROR))?;
    
    Ok(Json(result))
}

#[handler]
async fn update_region(
    db: Data<&DatabaseConnection>, 
    Path(slug): Path<String>,
    Json(payload): Json<UpdateRegion>
) -> Result<Json<RegionModel>> {
    // Find the region
    let region = Region::find()
        .filter(region::Column::Slug.eq(&slug))
        .one(db.0)
        .await
        .map_err(|e| Error::from_string(e.to_string(), StatusCode::INTERNAL_SERVER_ERROR))?;
    
    let region = match region {
        Some(region) => region,
        None => return Err(Error::from_string("Region not found", StatusCode::NOT_FOUND)),
    };
    
    // Update the region
    let mut region_model: region::ActiveModel = region.into();
    
    if let Some(katakana) = payload.katakana {
        region_model.katakana = Set(katakana);
    }
    
    if let Some(english) = payload.english {
        region_model.english = Set(english);
    }
    
    let updated_region = region_model.update(db.0)
        .await
        .map_err(|e| Error::from_string(e.to_string(), StatusCode::INTERNAL_SERVER_ERROR))?;
    
    Ok(Json(updated_region))
}

#[handler]
async fn delete_region(db: Data<&DatabaseConnection>, Path(slug): Path<String>) -> Result<()> {
    // Find the region
    let region = Region::find()
        .filter(region::Column::Slug.eq(&slug))
        .one(db.0)
        .await
        .map_err(|e| Error::from_string(e.to_string(), StatusCode::INTERNAL_SERVER_ERROR))?;
    
    let region = match region {
        Some(region) => region,
        None => return Err(Error::from_string("Region not found", StatusCode::NOT_FOUND)),
    };
    
    // Delete the region
    let region_model: region::ActiveModel = region.into();
    region_model.delete(db.0)
        .await
        .map_err(|e| Error::from_string(e.to_string(), StatusCode::INTERNAL_SERVER_ERROR))?;
    
    Ok(())
}