use sea_orm_migration::{prelude::*, schema::*};

use crate::m20240927_154643_create_streams::Streams;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Streams::Table)
                    .add_column_if_not_exists(text_null(Streams::Locale))
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("streams_locale_idx")
                    .table(Streams::Table)
                    .col(Streams::Locale)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Streams::Table)
                    .drop_column(Streams::Locale)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}
