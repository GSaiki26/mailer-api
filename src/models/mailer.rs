// Libs
use tonic::{Request, Response, Status};

use mailer_proto::{mailer_server::Mailer, Mail};

// Structs
#[derive(Default)]
pub struct MailerService;

// Implementations
#[tonic::async_trait]
impl Mailer for MailerService {
    async fn send_mail(&self, req: Request<Mail>) -> Result<Response<Mail>, Status> {
        let a = req.into_inner();
        let res = Response::<Mail>::new(a);
        Ok(res)
    }
}

// Modules
pub mod mailer_proto {
    tonic::include_proto!("mailer");
}
