use plex_api::MyPlexBuilder;
use rpassword::prompt_password_stdout;
use std::io::{stdin, stdout, BufRead, Write};

#[async_std::main]
async fn main() {
    print!("Username: ");
    stdout().flush().unwrap();

    let username = stdin().lock().lines().next().unwrap().unwrap();
    let password = prompt_password_stdout("Password: ").unwrap();

    let mut myplex_result = MyPlexBuilder::default()
        .set_username_and_password(&username, &password)
        .build()
        .await;

    if let Err(plex_api::Error::OtpRequired) = myplex_result {
        let otp = prompt_password_stdout("OTP: ").unwrap();
        myplex_result = MyPlexBuilder::default()
            .set_username_and_password(&username, &password)
            .set_otp(&otp)
            .build()
            .await;
    }

    let myplex = myplex_result.unwrap();

    println!("Auth token: {}", myplex.client().x_plex_token());
}
