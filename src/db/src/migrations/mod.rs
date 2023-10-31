pub use sea_orm_migration::prelude::*;

mod m20232910_000001_initial;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m20232910_000001_initial::Migration)]
    }
}
