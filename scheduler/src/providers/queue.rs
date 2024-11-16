use models::mail;
use sea_orm::{
    sea_query::SimpleExpr, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter,
};

pub async fn get_mails_to_send<'a>(db: &DatabaseConnection) -> Result<Vec<mail::Model>, DbErr> {
    mail::Entity::find().filter(get_mail_filter()).all(db).await
}

fn get_mail_filter() -> SimpleExpr {
    let current_time = chrono::Utc::now();

    mail::Column::WasSent
        .eq(false)
        .and(mail::Column::ScheduledAt.lte(current_time))
}
