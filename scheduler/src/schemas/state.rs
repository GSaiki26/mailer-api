use lettre::{AsyncSmtpTransport, Tokio1Executor};
use sea_orm::DatabaseConnection;

use crate::utils::MailSettings;

pub struct State<'a> {
    pub db: &'a DatabaseConnection,
    pub mail_settings: &'a MailSettings,
    pub sender: String,
    pub transport: &'a AsyncSmtpTransport<Tokio1Executor>,
}
