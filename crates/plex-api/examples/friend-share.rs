use plex_api::{
    sharing::{Filters, Permissions, ShareableLibrary, ShareableServer, User},
    MyPlexBuilder,
};
use rpassword::prompt_password;
use std::io::{stdin, stdout, BufRead, Write};

#[async_std::main]
async fn main() {
    let token = prompt_password("Token: ").unwrap();
    let myplex = MyPlexBuilder::default()
        .set_token(token)
        .build()
        .await
        .unwrap();

    print!("Friend's username or email: ");
    stdout().flush().unwrap();

    let friend_identifier = stdin().lock().lines().next().unwrap().unwrap();

    print!("Machine ID to share: ");
    stdout().flush().unwrap();

    let machine_identifier = stdin().lock().lines().next().unwrap().unwrap();

    let server_info = myplex.server_info(&machine_identifier).await.unwrap();

    if server_info.library_sections.is_empty() {
        panic!("The server has no libraries");
    }

    println!();

    println!("Server has the following libraries:");

    for library in &server_info.library_sections {
        println!("{}. {} ({})", library.key, library.title, library.r#type);
    }

    println!();

    print!("Comma-separated library keys to share (or 'all'): ");
    stdout().flush().unwrap();

    let mut libraries = stdin().lock().lines().next().unwrap().unwrap();

    if libraries == "all" {
        libraries = server_info
            .library_sections
            .iter()
            .map(|l| l.id.to_string())
            .collect::<Vec<String>>()
            .join(",");
    }

    let libraries: Vec<ShareableLibrary> = libraries
        .split(',')
        .map(|key| ShareableLibrary::LibraryId(key.trim()))
        .collect();

    let friend = myplex
        .sharing()
        .unwrap()
        .share(
            User::UsernameOrEmail(&friend_identifier),
            ShareableServer::MachineIdentifier(&machine_identifier),
            &libraries,
            Permissions::default(),
            Filters::default(),
        )
        .await
        .unwrap();

    println!("Invite sent! Friend id is {}", friend.id);
}
