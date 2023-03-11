use plex_api::{sharing::InviteStatus, MyPlexBuilder};
use rpassword::prompt_password;
use std::io::{stdin, stdout, BufRead, Write};

#[async_std::main]
async fn main() {
    let token = prompt_password("Token: ").unwrap();
    print!("Friend's username or friendly_name to delete: ");
    stdout().flush().unwrap();

    let username = stdin().lock().lines().next().unwrap().unwrap();

    let myplex = MyPlexBuilder::default()
        .set_token(token)
        .build()
        .await
        .unwrap();

    let friends = myplex
        .sharing()
        .friends(InviteStatus::Accepted)
        .await
        .unwrap();
    let mut friend = friends.into_iter().find(|friend| friend.title == username);

    if friend.is_none() {
        let friends = myplex
            .sharing()
            .friends(InviteStatus::PendingReceived)
            .await
            .unwrap();
        friend = friends.into_iter().find(|friend| friend.title == username);
    }

    if friend.is_none() {
        let friends = myplex
            .sharing()
            .friends(InviteStatus::PendingSent)
            .await
            .unwrap();
        friend = friends.into_iter().find(|friend| friend.title == username);
    }

    if let Some(friend) = friend {
        friend.delete().await.unwrap();
        println!("The friend was deleted!");
    } else {
        eprintln!("Unable to find a friend with username '{username}'.");
    }
}
