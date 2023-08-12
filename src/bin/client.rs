use anyhow::Error;
use clap::Parser;
use clap_stdin::MaybeStdin;
use examine::{examine_service_client::ExamineServiceClient, ExamineRequest};
use tonic::Request;

/// `client` checks user agent string(s)and emits whether to Allow or Block for each one.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// User-Agent string of the browser. Use `-` to indicate reading from stdin instead.
    user_agent: MaybeStdin<String>,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Parse command-line arguments
    let args = Args::parse();
    // There may be multiple user agents separated by newlines
    let user_agents: Vec<String> = args
        .user_agent
        .split("\n")
        .map(|s| s.to_string())
        .filter(|s| !s.is_empty())
        .collect();

    // Set up the client
    let mut client = ExamineServiceClient::connect("http://127.0.0.1:8080").await?;

    for user_agent in user_agents {
        let request = Request::new(ExamineRequest { user_agent });
        let response = client.examine(request).await?;
        let action = response.into_inner().action();
        // For this client we will output Rust's debug representation of the action, one per line.
        // For a real project we may need to marshal this into some sort of specific output format.
        println!("{action:?}");
    }

    Ok(())
}
