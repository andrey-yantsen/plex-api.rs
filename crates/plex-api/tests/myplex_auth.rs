use mockito::{mock, Matcher};
use plex_api::{
    url::{MYPLEX_SIGNIN_PATH, MYPLEX_SIGNOUT_PATH, MYPLEX_USER_INFO_PATH},
    Error, MyPlexBuilder,
};

#[plex_api_test_helper::async_offline_test]
async fn signin_free_user(client: Client) {
    let m = mock("POST", MYPLEX_SIGNIN_PATH)
        .with_status(201)
        .with_header("content-type", "text/json")
        .with_body_from_file("tests/files/myplex/api/v2/user/user_info_free.json")
        .match_body(Matcher::AllOf(vec![
            Matcher::UrlEncoded("login".to_string(), "username".to_string()),
            Matcher::UrlEncoded("password".to_string(), "password".to_string()),
            Matcher::UrlEncoded("mememberMe".to_string(), "true".to_string()),
        ]))
        .create();

    let plex_result = MyPlexBuilder::default()
        .set_client(client.set_x_plex_token("".to_owned()))
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

#[plex_api_test_helper::async_offline_test]
async fn signin_plexpass_user(client: Client) {
    let m = mock("POST", MYPLEX_SIGNIN_PATH)
        .with_status(201)
        .with_header("content-type", "text/json")
        .with_body_from_file("tests/files/myplex/api/v2/user/user_info_plexpass.json")
        .match_body(Matcher::AllOf(vec![
            Matcher::UrlEncoded("login".to_string(), "username".to_string()),
            Matcher::UrlEncoded("password".to_string(), "password".to_string()),
            Matcher::UrlEncoded("mememberMe".to_string(), "true".to_string()),
        ]))
        .create();

    let plex_result = MyPlexBuilder::default()
        .set_client(client.set_x_plex_token("".to_owned()))
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

#[plex_api_test_helper::async_offline_test]
async fn signin_without_required_otp(client: Client) {
    let m = mock("POST", MYPLEX_SIGNIN_PATH)
        .with_status(401)
        .with_header("content-type", "text/json")
        .with_body(
            "{\"errors\":[{\"code\": 1029, \"message\": \"OTP required\", \"status\": 401}]}",
        )
        .match_body(Matcher::AllOf(vec![
            Matcher::UrlEncoded("login".to_string(), "username".to_string()),
            Matcher::UrlEncoded("password".to_string(), "password".to_string()),
            Matcher::UrlEncoded("mememberMe".to_string(), "true".to_string()),
        ]))
        .create();

    let plex_result = MyPlexBuilder::default()
        .set_client(client.set_x_plex_token("".to_owned()))
        .set_username_and_password("username", "password")
        .build()
        .await;
    m.assert();

    let err = plex_result.err().expect("error expected");

    assert!(matches!(err, Error::OtpRequired), "unexpected error");
}

#[plex_api_test_helper::async_offline_test]
async fn signin_with_otp(client: Client) {
    let m = mock("POST", MYPLEX_SIGNIN_PATH)
        .with_status(201)
        .with_header("content-type", "text/json")
        .with_body_from_file("tests/files/myplex/api/v2/user/user_info_free.json")
        .match_body(Matcher::AllOf(vec![
            Matcher::UrlEncoded("login".to_string(), "username".to_string()),
            Matcher::UrlEncoded("password".to_string(), "password".to_string()),
            Matcher::UrlEncoded("mememberMe".to_string(), "true".to_string()),
            Matcher::UrlEncoded("verificationCode".to_string(), "123456".to_string()),
        ]))
        .create();

    let plex_result = MyPlexBuilder::default()
        .set_client(client.set_x_plex_token("".to_owned()))
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

#[plex_api_test_helper::async_offline_test]
async fn signin_with_token(client: Client) {
    let m = mock("GET", MYPLEX_USER_INFO_PATH)
        .with_status(200)
        .with_header("content-type", "text/json")
        .with_body_from_file("tests/files/myplex/api/v2/user/user_info_free.json")
        .match_header("X-Plex-Token", "auth_token_offline_signin_with_token")
        .create();

    let plex_result = MyPlexBuilder::default().set_client(client).build().await;
    m.assert();

    let plex = plex_result.expect("failed to login");

    assert_eq!(
        plex.client().x_plex_token(),
        "auth_token",
        "unexpected auth token"
    );
}

#[plex_api_test_helper::async_offline_test]
async fn signin_with_token_free_guest(client: Client) {
    let m = mock("GET", MYPLEX_USER_INFO_PATH)
        .with_status(200)
        .with_header("content-type", "text/json")
        .with_body_from_file("tests/files/myplex/api/v2/user/user_info_free_guest.json")
        .match_header(
            "X-Plex-Token",
            "auth_token_offline_signin_with_token_free_guest",
        )
        .create();

    let plex_result = MyPlexBuilder::default().set_client(client).build().await;
    m.assert();

    let plex = plex_result.expect("failed to login");

    assert_eq!(
        plex.client().x_plex_token(),
        "auth_token",
        "unexpected auth token"
    );
}

#[plex_api_test_helper::async_offline_test]
async fn signin_with_token_plexpass_guest(client: Client) {
    let m = mock("GET", MYPLEX_USER_INFO_PATH)
        .with_status(200)
        .with_header("content-type", "text/json")
        .with_body_from_file("tests/files/myplex/api/v2/user/user_info_plexpass_guest.json")
        .match_header(
            "X-Plex-Token",
            "auth_token_offline_signin_with_token_plexpass_guest",
        )
        .create();

    let plex_result = MyPlexBuilder::default().set_client(client).build().await;
    m.assert();

    let plex = plex_result.expect("failed to login");

    assert_eq!(
        plex.client().x_plex_token(),
        "auth_token",
        "unexpected auth token"
    );
}

#[plex_api_test_helper::async_offline_test]
async fn signout(myplex: MyPlex) {
    let m = mock("DELETE", MYPLEX_SIGNOUT_PATH)
        .with_status(204)
        .with_header("content-type", "text/json")
        .match_body("")
        .create();

    let signout_result = myplex.signout().await;
    m.assert();

    signout_result.expect("failed to signout");
}

#[plex_api_test_helper::async_offline_test]
async fn signin_failures(client: Client) {
    let client2 = client.clone();

    let plex_result = MyPlexBuilder::default()
        .set_client(client)
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
