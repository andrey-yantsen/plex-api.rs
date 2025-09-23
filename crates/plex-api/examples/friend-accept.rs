use plex_api::{sharing::InviteStatus::PendingReceived, MyPlexBuilder};
use rpassword::prompt_password;
use std::io::{stdin, stdout, BufRead, Write};

#[tokio::main]
async fn main() {
    let token = prompt_password("Token: ").unwrap();
    print!("Friend's username or friendly_name to accept invitation: ");
    stdout().flush().unwrap();

    let username = stdin().lock().lines().next().unwrap().unwrap();

    let myplex = MyPlexBuilder::default()
        .set_token(token)
        .build()
        .await
        .unwrap();

    let friends = myplex
        .sharing()
        .unwrap()
        .friends(PendingReceived)
        .await
        .unwrap();
    let friend = friends.into_iter().find(|friend| friend.title == username);

    if let Some(friend) = friend {
        friend.accept().await.unwrap();
        println!("The invitation was accepts!");
    } else {
        eprintln!("Unable to find a friend with username '{username}'.");
    }
}
