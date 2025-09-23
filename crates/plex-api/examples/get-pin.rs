use plex_api::MyPlexBuilder;
use rpassword::prompt_password;

#[tokio::main]
async fn main() {
    let token = prompt_password("Token: ").unwrap();

    let plex = MyPlexBuilder::default()
        .set_token(token)
        .build()
        .await
        .unwrap();

    let pin_manager = plex.pin_manager().unwrap();
    let pin = pin_manager.pin().await.unwrap();

    println!("Code: {}", pin.code());
    println!("Please link it on http://plex.tv/link/");

    loop {
        match pin.check().await {
            Ok(pin) => {
                println!(
                    "The pin was linked, auth token is '{}'",
                    pin.auth_token.unwrap()
                );
                break;
            }
            Err(plex_api::Error::PinExpired) => {
                println!("The pin has expired, please request a new one");
                break;
            }
            _ => tokio::time::sleep(std::time::Duration::from_secs(1)).await,
        }
    }
}
