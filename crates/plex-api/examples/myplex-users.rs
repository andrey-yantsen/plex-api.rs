use plex_api::MyPlexBuilder;
use rpassword::prompt_password;
use secrecy::ExposeSecret;
use std::io::{stdin, stdout, BufRead, Write};

#[tokio::main]
async fn main() {
    let token = prompt_password("Token: ").unwrap();
    stdout().flush().unwrap();

    let myplex = MyPlexBuilder::default()
        .set_token(token)
        .build()
        .await
        .unwrap();

    let home = myplex.home().unwrap();
    let users = home.users().await.unwrap();

    if users.len() == 1 {
        println!("Only one user in your Plex Home! I can't do anything here.");
        return;
    }

    println!("Please select a user to switch to:");

    for (idx, user) in users.iter().enumerate() {
        println!(
            "{idx}. {title}; restricted: {restricted}; has pin: {has_pin}",
            idx = idx + 1,
            title = user.title,
            restricted = user.restricted,
            has_pin = user.protected
        );
    }

    let idx = stdin().lock().lines().next().unwrap().unwrap();
    let idx: usize = idx.parse().unwrap();

    if idx > users.len() + 1 || idx < 1 {
        eprintln!("Don't be like that");
        return;
    }

    let target_user = &users[idx - 1];

    let mut pin = "".to_string();

    if target_user.protected {
        pin = prompt_password("Please enter pin for the user: ").unwrap();
    }

    let myplex = home
        .switch_user(myplex, target_user, Some(&pin))
        .await
        .unwrap();

    println!(
        "Auth token for {} is '{}'",
        target_user.title,
        myplex.account().unwrap().auth_token.expose_secret()
    );
}
