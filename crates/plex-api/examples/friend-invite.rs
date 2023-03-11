use plex_api::{sharing::User, MyPlexBuilder};
use rpassword::prompt_password;
use std::io::{stdin, stdout, BufRead, Write};

#[async_std::main]
async fn main() {
    let token = prompt_password("Token: ").unwrap();
    print!("Friend's username or email: ");
    stdout().flush().unwrap();

    let identifier = stdin().lock().lines().next().unwrap().unwrap();

    let myplex = MyPlexBuilder::default()
        .set_token(token)
        .build()
        .await
        .unwrap();

    let friend = myplex
        .sharing()
        .invite(User::UsernameOrEmail(&identifier))
        .await
        .unwrap();

    println!("Invite sent! Friend id is {}", friend.id);
}
