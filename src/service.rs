use std::result::Result;

use tonic::{async_trait, Request, Response, Status};

use crate::{examine_service_server::ExamineService, Action, ExamineRequest, ExamineResponse};

/// The struct representing the `Examine` service.
///
/// # Example
///
/// Here is how you could run this service.
///
/// ```rust
/// # use anyhow::Error;
/// # use examine::{examine_service_server::*, service::Examine};
/// # use tonic::transport::Server;
/// # use std::net::SocketAddr;
/// #[tokio::main]
/// async fn main() -> Result<(), Error> {
///     let addr: SocketAddr = "127.0.0.1:8080".parse()?;
///     let service = ExamineServiceServer::new(Examine {});
///
///     // (Commented out so that the doc-tests don't block running an actual server)
///     // Server::builder().add_service(service).serve(addr).await?;
///     Ok(())
/// }
/// ```
/// NOTE: The generated code requires you to create a struct that implements the [`ExamineService`]
/// trait. I would have preferred for this struct to have been named `ExamineService`...but that is
/// already the trait's name! 🤔 I might want to find a better name than `Examine` if this were a
/// real project.
pub struct Examine;

#[async_trait]
impl ExamineService for Examine {
    async fn examine(
        &self,
        request: Request<ExamineRequest>,
    ) -> Result<Response<ExamineResponse>, Status> {
        // Get the User-Agent header supplied in the request
        let user_agent = request.into_inner().user_agent;

        // The default should *probably* shouldn't be the unused `Action::Unspecified`, so I
        // arbitrarily chose `Action::Allow` as the default.
        let mut action = Action::Allow;

        // Presumably, a real application would have much more sophisticated logic here.
        if user_agent.contains("Firefox/") {
            action = Action::Allow;
        }
        if user_agent.contains("Safari/") {
            action = Action::Block;
        }

        // This is the place to emit proper logging, tracing, metrics, etc. but since this is a toy
        // project, we'll just emit some info to the console.
        println!("{action:?} -> {user_agent}");

        // Package up and return the response
        Ok(Response::new(ExamineResponse {
            action: action.into(),
        }))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use anyhow::Error;

    #[tokio::test]
    async fn examine_logic_is_correct() -> Result<(), Error> {
        let examine_service = Examine {};
        let test_cases: Vec<(Action, &str)> = vec![
            (Action::Block, "Mozilla/5.0 (Macintosh; Intel Mac OS X 13_5) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/16.5 Safari/605.1.15"),
            (Action::Allow, "Mozilla/5.0 (Macintosh; Intel Mac OS X 13.5; rv:109.0) Gecko/20100101 Firefox/116.0"),
        ];
        for (expected_action, user_agent) in test_cases {
            let request = Request::new(ExamineRequest {
                user_agent: user_agent.into(),
            });
            let response = examine_service.examine(request).await?;
            let action = response.into_inner().action();
            assert_eq!(expected_action, action);
        }
        Ok(())
    }
}
