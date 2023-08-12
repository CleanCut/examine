use std::time::Duration;

use anyhow::Error;
use examine::{
    examine_service_client::ExamineServiceClient, examine_service_server::ExamineServiceServer,
    service::Examine, Action, ExamineRequest,
};
use tokio::{spawn, time::sleep};
use tonic::{transport::Server, Request};

#[tokio::test]
async fn end_to_end_tests() -> Result<(), Error> {
    // Start the server on a different port than the real one
    let addr = "127.0.0.1:9500".parse()?;
    let service = ExamineServiceServer::new(Examine {});

    // Start the server asynchronously, and wait 1 second for it to actually start serving. For a
    // real project it would be better to wait for a health check to succeed, or abort if a timeout
    // is hit.
    let server_handle = spawn(Server::builder().add_service(service).serve(addr));
    sleep(Duration::from_secs(1)).await;

    // Connect a client to the server
    let mut client = ExamineServiceClient::connect(format!("http://{}", addr)).await?;

    // Run through all the test cases
    let test_cases: Vec<(Action, &str)> = vec![
        (Action::Block, "Mozilla/5.0 (Macintosh; Intel Mac OS X 13_5) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/16.5 Safari/605.1.15"),
        (Action::Allow, "Mozilla/5.0 (Macintosh; Intel Mac OS X 13.5; rv:109.0) Gecko/20100101 Firefox/116.0"),
    ];
    for (expected_action, user_agent) in test_cases {
        let request = Request::new(ExamineRequest {
            user_agent: user_agent.into(),
        });
        let response = client.examine(request).await?;
        let action = response.into_inner().action();
        assert_eq!(expected_action, action);
    }

    // Stop the server
    server_handle.abort();
    Ok(())
}
