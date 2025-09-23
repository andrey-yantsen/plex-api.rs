use plex_api::{device::Device, MyPlexBuilder};
use rpassword::prompt_password;
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

    let device_manager = myplex.device_manager().unwrap();
    let devices = device_manager.devices().await.unwrap();

    if devices.is_empty() {
        eprintln!("You have no devices");
        return;
    }

    let devices = devices
        .into_iter()
        .filter(|device| device.is_server())
        .collect::<Vec<Device>>();

    let device = {
        if devices.len() == 1 {
            devices.first().unwrap()
        } else {
            println!("Please selected the server you want to connect to:");
            for (idx, d) in devices.iter().enumerate() {
                println!("{}. {}", idx + 1, d.name());
            }
            let idx = stdin().lock().lines().next().unwrap().unwrap();
            let idx: usize = idx.parse().unwrap();
            if idx > devices.len() + 1 || idx < 1 {
                eprintln!("Don't be like that");
                return;
            }
            devices.get(idx - 1).unwrap()
        }
    };
    let device = match device.connect().await.unwrap() {
        plex_api::device::DeviceConnection::Server(srv) => srv,
        _ => panic!("HOW?"),
    };

    println!("Please enter item id:");
    let id = stdin().lock().lines().next().unwrap().unwrap();

    _ = dbg!(device.item_by_id(&id).await);
}
