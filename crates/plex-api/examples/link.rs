use plex_api::MyPlexBuilder;
use rpassword::prompt_password_stdout;

#[async_std::main]
async fn main() {
    let token = prompt_password_stdout("Token: ").unwrap();
    let code = prompt_password_stdout("Code: ").unwrap();

    MyPlexBuilder::default()
        .set_token(&token)
        .build()
        .await
        .unwrap()
        .pin_manager()
        .link(&code)
        .await
        .unwrap();
}
