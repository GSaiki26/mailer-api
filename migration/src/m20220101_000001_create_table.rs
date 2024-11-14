use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Mail::Table)
                    .if_not_exists()
                    .col(pk_uuid(Mail::Id))
                    .col(array(Mail::To, ColumnType::String(StringLen::N(320))).not_null())
                    .col(array(Mail::Cc, ColumnType::String(StringLen::N(320))).not_null())
                    .col(array(Mail::Bcc, ColumnType::String(StringLen::N(320))).not_null())
                    .col(string(Mail::Subject).not_null())
                    .col(string(Mail::Body).not_null())
                    .col(date_time(Mail::ScheduledAt).not_null())
                    .col(boolean(Mail::WasSent).not_null())
                    .col(date_time(Mail::CreatedAt).not_null())
                    .col(date_time(Mail::UpdatedAt).not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Attachment::Table)
                    .if_not_exists()
                    .col(pk_uuid(Attachment::Id))
                    .col(uuid(Attachment::MailId).not_null())
                    .col(string(Attachment::Filename).not_null())
                    .col(binary(Attachment::Content).not_null())
                    .col(string(Attachment::ContentType).not_null())
                    .col(date_time(Attachment::CreatedAt).not_null())
                    .col(date_time(Attachment::UpdatedAt).not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Mail::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Mail {
    Table,
    Id,
    To,
    Cc,
    Bcc,
    Subject,
    Body,
    ScheduledAt,
    WasSent,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum Attachment {
    Table,
    Id,
    MailId,
    Filename,
    Content,
    ContentType,
    CreatedAt,
    UpdatedAt,
}
