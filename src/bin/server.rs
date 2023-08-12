use anyhow::Error;
use examine::{examine_service_server::*, service::Examine};
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let addr = "127.0.0.1:8080".parse()?;
    let service = ExamineServiceServer::new(Examine {});

    Server::builder().add_service(service).serve(addr).await?;
    Ok(())
}
