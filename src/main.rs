// Libs
use models::mailer::MailerService;

use models::mailer::mailer_proto::mailer_server::MailerServer;

mod models;

// Main
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:50051".parse::<std::net::SocketAddr>()?;
    let greeter = MailerService;

    println!("Server starting at {}", addr);
    tonic::transport::Server::builder()
        .add_service(MailerServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
