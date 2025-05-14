pub use sea_orm_migration::prelude::*;

mod m20250426_024738_add_regions_table;
mod m20250508_022912_seed_regions_data;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250426_024738_add_regions_table::Migration),
            Box::new(m20250508_022912_seed_regions_data::Migration),
        ]
    }
}
