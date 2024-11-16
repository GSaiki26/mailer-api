use std::sync::Arc;

use crate::services::MailService;

#[derive(Clone)]
pub struct APIState {
    pub mail_svc: Arc<MailService>,
}
