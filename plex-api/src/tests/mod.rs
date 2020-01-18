#[cfg(any(
    feature = "test_connect_authenticated",
    feature = "test_connect_anonymous"
))]
use crate::Server;

mod headers;
mod media_container;

#[cfg(any(
    feature = "test_connect_authenticated",
    feature = "test_connect_anonymous"
))]
mod my_plex;

#[cfg(any(
    feature = "test_connect_authenticated",
    feature = "test_connect_anonymous"
))]
mod server;

#[cfg(feature = "test_connect_authenticated")]
async fn get_server_authenticated() -> Server {
    use std::env;
    let srv: Result<Server, _> = {
        let server_url = env::var("PLEX_API_SERVER_URL").expect("Server url not specified");
        let auth_token = env::var("PLEX_API_AUTH_TOKEN").expect("Auth token not specified");
        Server::connect_auth(&server_url, &auth_token).await
    };
    assert!(srv.is_ok(), "Unable to connect to server: {:?}", srv.err());
    srv.ok().unwrap()
}

#[cfg(feature = "test_connect_anonymous")]
async fn get_server_anonymous() -> Server {
    use std::env;
    let srv: Result<Server, _> = {
        let server_url = env::var("PLEX_API_SERVER_URL").expect("Server url not specified");
        Server::connect(&server_url).await
    };
    assert!(srv.is_ok(), "Unable to connect to server: {:?}", srv.err());
    srv.ok().unwrap()
}

#[cfg(any(
    feature = "test_connect_authenticated",
    feature = "test_connect_anonymous"
))]
pub(crate) mod retry {
    use crate::PlexApiError;
    use futures_retry::{ErrorHandler, RetryPolicy};
    use std::time::Duration;

    pub(crate) struct FutureRetryHandler<D> {
        max_attempts: usize,
        current_attempt: usize,
        display_name: D,
    }

    impl<D> FutureRetryHandler<D> {
        pub fn new(max_attempts: usize, display_name: D) -> Self {
            FutureRetryHandler {
                max_attempts,
                current_attempt: 0,
                display_name,
            }
        }
    }

    impl<D> ErrorHandler<PlexApiError> for FutureRetryHandler<D>
    where
        D: ::std::fmt::Display,
    {
        type OutError = PlexApiError;

        fn handle(&mut self, e: PlexApiError) -> RetryPolicy<PlexApiError> {
            self.current_attempt += 1;
            if self.current_attempt >= self.max_attempts {
                eprintln!(
                    "[{}] All attempts ({}) have been used",
                    self.display_name, self.max_attempts
                );
                return RetryPolicy::ForwardError(e);
            }
            eprintln!(
                "[{}] Attempt {}/{} has failed",
                self.display_name, self.current_attempt, self.max_attempts
            );
            match e {
                PlexApiError::ReqwestError { .. } => RetryPolicy::WaitRetry(Duration::from_secs(5)),
                _ => RetryPolicy::ForwardError(e),
            }
        }
    }
}
