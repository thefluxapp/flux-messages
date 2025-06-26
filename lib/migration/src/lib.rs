pub use sea_orm_migration::prelude::*;

mod entities;
mod m20240927_153528_create_messages;
mod m20240927_154643_create_streams;
mod m20240927_160951_create_messages_streams;
mod m20240927_161006_create_streams_users;
mod m20240928_170119_create_indexes;
mod m20250609_151902_add_locale_to_messages;
mod m20250609_163004_add_locale_to_streams;
mod m20250626_141401_add_messages_count_to_streams;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240927_153528_create_messages::Migration),
            Box::new(m20240927_154643_create_streams::Migration),
            Box::new(m20240927_160951_create_messages_streams::Migration),
            Box::new(m20240927_161006_create_streams_users::Migration),
            Box::new(m20240928_170119_create_indexes::Migration),
            Box::new(m20250609_151902_add_locale_to_messages::Migration),
            Box::new(m20250609_163004_add_locale_to_streams::Migration),
            Box::new(m20250626_141401_add_messages_count_to_streams::Migration),
        ]
    }
}
