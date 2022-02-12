use plex_api::MyPlexBuilder;
use rpassword::prompt_password_stdout;

#[async_std::main]
async fn main() {
    let token = prompt_password_stdout("Token: ").unwrap();

    MyPlexBuilder::default()
        .set_token(&token)
        .build()
        .await
        .unwrap()
        .signout()
        .await
        .unwrap();
}
