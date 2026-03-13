pub use sea_orm_migration::prelude::*;

mod m20260312_214553_create_user_table;
mod m20260313_080357_create_totp_config;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20260312_214553_create_user_table::Migration),
            Box::new(m20260313_080357_create_totp_config::Migration),
        ]
    }
}
