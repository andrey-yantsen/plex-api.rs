macro_rules! test_case_online_anonymous {
    ($testing_function_name:ident) => {
        paste! {
            #[cfg(feature = "test_connect_anonymous")]
            #[tokio::test]
            async fn [<$testing_function_name _anonymous>]() {
                let srv = crate::tests::get_server_anonymous().await;
                $testing_function_name(srv).await;
            }
        }
    };
}

macro_rules! test_case_online_authenticated {
    ($testing_function_name:ident) => {
        paste! {
            #[cfg(feature = "test_connect_authenticated")]
            #[tokio::test]
            async fn [<$testing_function_name _authenticated>]() {
                let srv = crate::tests::get_server_authenticated().await;
                $testing_function_name(srv).await;
            }
        }
    };
}

macro_rules! test_case_online_all {
    ($testing_function_name:ident) => {
        test_case_online_anonymous!($testing_function_name);
        test_case_online_authenticated!($testing_function_name);
    };
}

mod headers;
mod media_container;

mod library;
mod my_plex;
mod server;

#[cfg(any(
    feature = "test_connect_authenticated",
    feature = "test_connect_anonymous"
))]
fn set_client_identifier_from_env() {
    use std::env;

    let client_id = env::var("X_PLEX_CLIENT_IDENTIFIER");
    if let Ok(client_id) = client_id {
        use crate::X_PLEX_CLIENT_IDENTIFIER;
        let mut client_identifier = X_PLEX_CLIENT_IDENTIFIER.write().unwrap();
        *client_identifier = client_id;
    }
}

#[cfg(feature = "test_connect_authenticated")]
async fn get_server_authenticated() -> crate::Server {
    use std::env;
    let srv: Result<crate::Server, _> = {
        let server_url = env::var("PLEX_API_SERVER_URL").expect("Server url not specified");
        let auth_token = env::var("PLEX_API_AUTH_TOKEN").expect("Auth token not specified");
        set_client_identifier_from_env();
        crate::Server::connect(&server_url, &auth_token).await
    };
    assert!(srv.is_ok(), "Unable to connect to server: {:?}", srv.err());
    srv.unwrap()
}

#[cfg(feature = "test_connect_anonymous")]
async fn get_server_anonymous() -> crate::Server {
    use std::env;
    let srv: Result<crate::Server, _> = {
        let server_url = env::var("PLEX_API_SERVER_URL").expect("Server url not specified");
        set_client_identifier_from_env();
        crate::Server::connect(&server_url, "").await
    };
    assert!(srv.is_ok(), "Unable to connect to server: {:?}", srv.err());
    srv.unwrap()
}

#[cfg(any(feature = "test_connect_authenticated"))]
pub(crate) mod retry {
    use crate::PlexApiError;
    use futures_retry::{ErrorHandler, RetryPolicy};
    use std::time::Duration;

    pub(crate) struct FutureRetryHandler<D> {
        max_attempts: usize,
        display_name: D,
    }

    impl<D> FutureRetryHandler<D> {
        pub const fn new(max_attempts: usize, display_name: D) -> Self {
            FutureRetryHandler {
                max_attempts,
                display_name,
            }
        }
    }

    impl<D> ErrorHandler<PlexApiError> for FutureRetryHandler<D>
    where
        D: ::std::fmt::Display,
    {
        type OutError = PlexApiError;

        fn handle(&mut self, current_attempt: usize, e: PlexApiError) -> RetryPolicy<PlexApiError> {
            if current_attempt >= self.max_attempts {
                eprintln!(
                    "[{}] All attempts ({}) have been used",
                    self.display_name, self.max_attempts
                );
                return RetryPolicy::ForwardError(e);
            }
            eprintln!(
                "[{}] Attempt {}/{} has failed",
                self.display_name, current_attempt, self.max_attempts
            );
            match e {
                PlexApiError::ReqwestError { .. } => RetryPolicy::WaitRetry(Duration::from_secs(5)),
                _ => RetryPolicy::ForwardError(e),
            }
        }
    }
}
