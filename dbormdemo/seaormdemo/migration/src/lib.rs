pub use sea_orm_migration::prelude::*;

mod m20230710_011100_create_bakery_table;
mod m20230710_011111_create_chef_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20230710_011100_create_bakery_table::Migration),
            Box::new(m20230710_011111_create_chef_table::Migration),
        ]
    }
}
