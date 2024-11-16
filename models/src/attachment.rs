use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Clone, Debug, DeriveEntityModel, Deserialize, PartialEq, Eq, Serialize, Validate)]
#[sea_orm(table_name = "attachment")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,

    pub mail_id: Uuid,

    pub filename: String,
    pub content: Vec<u8>,
    pub content_type: String,

    #[serde(default)]
    pub created_at: DateTimeUtc,

    #[serde(default)]
    pub updated_at: DateTimeUtc,
}

#[derive(Debug, DeriveRelation, EnumIter)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::mail::Entity",
        from = "Column::MailId",
        to = "super::mail::Column::Id"
    )]
    Mail,
}

impl Related<super::mail::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Mail.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
