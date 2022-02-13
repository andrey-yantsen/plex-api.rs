use plex_api::PinManager;

#[async_std::main]
async fn main() {
    let pin_manager = PinManager::default();
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
            _ => async_std::task::sleep(std::time::Duration::from_secs(1)).await,
        }
    }
}
