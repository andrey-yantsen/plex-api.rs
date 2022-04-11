mod fixtures;

mod offline {
    use super::fixtures::offline::client::*;
    use super::fixtures::offline::myplex::*;
    use super::fixtures::offline::Mocked;
    use httpmock::Method::{DELETE, GET, POST};
    use plex_api::MyPlex;
    use plex_api::{
        url::{MYPLEX_SIGNIN_PATH, MYPLEX_SIGNOUT_PATH, MYPLEX_USER_INFO_PATH},
        Error, HttpClient, MyPlexBuilder,
    };

    #[plex_api_test_helper::offline_test]
    async fn signin_free_user(client_anonymous: Mocked<HttpClient>) {
        let (client_anonymous, mock_server) = client_anonymous.split();

        let m = mock_server.mock(|when, then| {
            when.method(POST)
                .path(MYPLEX_SIGNIN_PATH)
                .x_www_form_urlencoded_tuple("login", "username")
                .x_www_form_urlencoded_tuple("password", "password")
                .x_www_form_urlencoded_tuple("rememberMe", "true");
            then.status(200)
                .header("content-type", "text/json")
                .body_from_file("tests/mocks/myplex/api/v2/user/user_info_free.json");
        });

        let plex_result = MyPlexBuilder::default()
            .set_client(client_anonymous)
            .set_username_and_password("username", "password")
            .build()
            .await;
        m.assert();

        let plex = plex_result.expect("failed to login");

        assert_eq!(
            plex.client().x_plex_token(),
            "auth_token",
            "unexpected auth token"
        );
    }

    #[plex_api_test_helper::offline_test]
    async fn signin_plexpass_user(client_anonymous: Mocked<HttpClient>) {
        let (client_anonymous, mock_server) = client_anonymous.split();

        let m = mock_server.mock(|when, then| {
            when.method(POST)
                .path(MYPLEX_SIGNIN_PATH)
                .x_www_form_urlencoded_tuple("login", "username")
                .x_www_form_urlencoded_tuple("password", "password")
                .x_www_form_urlencoded_tuple("rememberMe", "true");
            then.status(200)
                .header("content-type", "text/json")
                .body_from_file("tests/mocks/myplex/api/v2/user/user_info_plexpass.json");
        });

        let plex_result = MyPlexBuilder::default()
            .set_client(client_anonymous)
            .set_username_and_password("username", "password")
            .build()
            .await;
        m.assert();

        let plex = plex_result.expect("failed to login");

        assert_eq!(
            plex.client().x_plex_token(),
            "auth_token",
            "unexpected auth token"
        );
    }

    #[plex_api_test_helper::offline_test]
    async fn signin_without_required_otp(client_anonymous: Mocked<HttpClient>) {
        let (client_anonymous, mock_server) = client_anonymous.split();

        let m = mock_server.mock(|when, then| {
            when.method(POST)
                .path(MYPLEX_SIGNIN_PATH)
                .x_www_form_urlencoded_tuple("login", "username")
                .x_www_form_urlencoded_tuple("password", "password")
                .x_www_form_urlencoded_tuple("rememberMe", "true");
            then.status(401)
                .header("content-type", "text/json")
                .body(r#"{"errors":[{"code": 1029, "message": "OTP required", "status": 401}]}"#);
        });

        let plex_result = MyPlexBuilder::default()
            .set_client(client_anonymous)
            .set_username_and_password("username", "password")
            .build()
            .await;
        m.assert();

        let err = plex_result.expect_err("error expected");

        assert!(matches!(err, Error::OtpRequired), "unexpected error");
    }

    #[plex_api_test_helper::offline_test]
    async fn signin_with_otp(client_anonymous: Mocked<HttpClient>) {
        let (client_anonymous, mock_server) = client_anonymous.split();

        let m = mock_server.mock(|when, then| {
            when.method(POST)
                .path(MYPLEX_SIGNIN_PATH)
                .x_www_form_urlencoded_tuple("login", "username")
                .x_www_form_urlencoded_tuple("password", "password")
                .x_www_form_urlencoded_tuple("rememberMe", "true")
                .x_www_form_urlencoded_tuple("verificationCode", "123456");
            then.status(200)
                .header("content-type", "text/json")
                .body_from_file("tests/mocks/myplex/api/v2/user/user_info_free.json");
        });

        let plex_result = MyPlexBuilder::default()
            .set_client(client_anonymous)
            .set_username_and_password("username", "password")
            .set_otp("123456")
            .build()
            .await;
        m.assert();

        let plex = plex_result.expect("failed to login");

        assert_eq!(
            plex.client().x_plex_token(),
            "auth_token",
            "unexpected auth token"
        );
    }

    #[plex_api_test_helper::offline_test]
    #[case::free_user("tests/mocks/myplex/api/v2/user/user_info_free.json")]
    #[case::free_user_managed_guest("tests/mocks/myplex/api/v2/user/user_info_free_guest.json")]
    #[case::plexpass_user("tests/mocks/myplex/api/v2/user/user_info_plexpass.json")]
    #[case::plexpass_user_managed_guest(
        "tests/mocks/myplex/api/v2/user/user_info_plexpass_guest.json"
    )]
    async fn signin_with_token(
        client_authenticated: Mocked<HttpClient>,
        #[case] mock_data_file: &str,
    ) {
        let (client_authenticated, mock_server) = client_authenticated.split();

        let m = mock_server.mock(|when, then| {
            when.method(GET)
                .path(MYPLEX_USER_INFO_PATH)
                .header("X-Plex-Token", "fixture_auth_token");
            then.status(200)
                .header("content-type", "text/json")
                .body_from_file(mock_data_file);
        });

        let plex_result = MyPlexBuilder::default()
            .set_client(client_authenticated)
            .build()
            .await;
        m.assert();

        let plex = plex_result.expect("failed to login");

        assert_eq!(
            plex.client().x_plex_token(),
            "auth_token",
            "unexpected auth token"
        );
    }

    #[plex_api_test_helper::offline_test]
    async fn signout(#[future] myplex: Mocked<MyPlex>) {
        let myplex = myplex.await;
        let (myplex, mock_server) = myplex.split();

        let m = mock_server.mock(|when, then| {
            when.method(DELETE)
                .path(MYPLEX_SIGNOUT_PATH)
                .header("X-Plex-Token", "auth_token");
            then.status(204).header("content-type", "text/json");
        });

        let signout_result = myplex.signout().await;
        m.assert();

        signout_result.expect("failed to signout");
    }

    #[plex_api_test_helper::offline_test]
    async fn signin_failures(client_anonymous: Mocked<HttpClient>) {
        let (client_anonymous, _) = client_anonymous.split();

        let client2 = client_anonymous.clone();

        let plex_result = MyPlexBuilder::default()
            .set_client(client_anonymous)
            .set_username_and_password("username", "password")
            .build()
            .await;
        assert!(
            plex_result.is_err(),
            "MyPlex::login() with set auth_token should fail"
        );

        let client3 = client2.clone();

        let plex_result = MyPlexBuilder::default()
            .set_client(client2)
            .set_username_and_password("username", "password")
            .set_otp("123456")
            .build()
            .await;
        assert!(
            plex_result.is_err(),
            "MyPlex::login_with_otp() with set auth_token should fail"
        );

        let client3 = client3.set_x_plex_token("".to_owned());
        let plex_result = MyPlexBuilder::default().set_client(client3).build().await;
        assert!(
            plex_result.is_err(),
            "MyPlex::new() without set auth_token should fail"
        );
    }
}
