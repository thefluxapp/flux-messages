use sea_orm_migration::{prelude::*, schema::*};

use crate::entities::{MessagesStreams, Streams};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Streams::Table)
                    .add_column_if_not_exists(big_integer_null(Streams::MessagesCount))
                    .to_owned(),
            )
            .await?;

        let update = Query::update()
            .table(Streams::Table)
            .value(
                Streams::MessagesCount,
                SimpleExpr::SubQuery(
                    None,
                    Box::new(
                        Query::select()
                            .expr(Expr::col(MessagesStreams::Id).count())
                            .from(MessagesStreams::Table)
                            .and_where(
                                Expr::col((MessagesStreams::Table, MessagesStreams::StreamId))
                                    .eq(Expr::col((Streams::Table, Streams::Id))),
                            )
                            .to_owned()
                            .into_sub_query_statement(),
                    ),
                ),
            )
            .to_owned();

        manager.exec_stmt(update).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Streams::Table)
                    .drop_column(Streams::MessagesCount)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use sea_orm_migration::prelude::*;

    use crate::entities::{MessagesStreams, Streams};

    #[test]
    fn check_default_value() {
        let update = Query::update()
            .table(Streams::Table)
            .value(
                Streams::MessagesCount,
                SimpleExpr::SubQuery(
                    None,
                    Box::new(
                        Query::select()
                            .expr(Expr::col(MessagesStreams::Id).count())
                            .from(MessagesStreams::Table)
                            .and_where(
                                Expr::col((MessagesStreams::Table, MessagesStreams::StreamId))
                                    .eq(Expr::col((Streams::Table, Streams::Id))),
                            )
                            .to_owned()
                            .into_sub_query_statement(),
                    ),
                ),
            )
            .to_owned();

        assert_eq!(
            update.to_string(PostgresQueryBuilder),
            r#"UPDATE "streams" SET "messages_count" = (SELECT COUNT("id") FROM "messages_streams" WHERE "messages_streams"."stream_id" = "streams"."id")"#
        );
    }
}
