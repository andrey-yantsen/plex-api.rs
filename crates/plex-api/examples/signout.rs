use plex_api::MyPlexBuilder;
use rpassword::prompt_password;

#[tokio::main]
async fn main() {
    let token = prompt_password("Token: ").unwrap();

    MyPlexBuilder::default()
        .set_token(token)
        .build()
        .await
        .unwrap()
        .signout()
        .await
        .unwrap();
}
