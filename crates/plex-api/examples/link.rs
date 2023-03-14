use plex_api::MyPlexBuilder;
use rpassword::prompt_password;

#[async_std::main]
async fn main() {
    let token = prompt_password("Token: ").unwrap();
    let code = prompt_password("Code: ").unwrap();

    MyPlexBuilder::default()
        .set_token(token)
        .build()
        .await
        .unwrap()
        .pin_manager()
        .unwrap()
        .link(&code)
        .await
        .unwrap();
}
