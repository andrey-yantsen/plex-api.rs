use plex_api::{
    device::Device,
    library::{
        Collection, FromMetadata, Library, MediaItem, MetadataItem, MovieLibrary, MusicLibrary,
        PhotoLibrary, Playlist, TVLibrary,
    },
    MyPlexBuilder,
};
use rpassword::prompt_password;
use std::io::{stdin, stdout, BufRead, Write};

async fn process_playlists<T: MediaItem + FromMetadata>(playlists: Vec<Playlist<T>>) {
    println!("Received {} playlist(s)", playlists.len());

    for playlist in playlists {
        println!("Processing '{}'", playlist.title());
        println!(
            "Received {} item(s)",
            playlist
                .children()
                .await
                .expect("failed to get children")
                .len()
        );
    }
}

async fn process_collections<T: FromMetadata>(collections: Vec<Collection<T>>) {
    println!("Received {} collection(s)", collections.len());
    for collection in collections {
        println!("Processing collection '{}'", collection.title());
        let children = collection
            .children()
            .await
            .expect("failed to load collection");
        println!("Received {} items", children.len());
    }
}

async fn process_movie_library(library: MovieLibrary) {
    let movies = library.movies().await.expect("failed to get movies");
    println!("Received {} movie(s)", movies.len());

    let collections = library
        .collections()
        .await
        .expect("failed to get collections");

    process_collections(collections).await;

    let playlists = library.playlists().await.expect("failed to get playlists");
    process_playlists(playlists).await;
}

async fn process_tv_library(library: TVLibrary) {
    let shows = library.shows().await.expect("failed to get shows");
    println!("Received {} show(s)", shows.len());

    for show in shows {
        println!("Processing show '{}'", show.title());
        println!(
            "Received {} season(s)",
            show.seasons().await.expect("failed to get seasons").len()
        );
        println!(
            "Received {} episode(s)",
            show.episodes().await.expect("failed to get episodes").len()
        );
    }

    let collections = library
        .collections()
        .await
        .expect("failed to get collections");
    process_collections(collections).await;

    let playlists = library.playlists().await.expect("failed to get playlists");
    process_playlists(playlists).await;
}

async fn process_music_library(library: MusicLibrary) {
    let artists = library.artists().await.expect("failed to get artists");
    println!("Received {} artist(s)", artists.len());

    for artist in artists {
        println!("Processing artist '{}'", artist.title());
        println!(
            "Received {} album(s)",
            artist.albums().await.expect("failed to get albums").len()
        );
    }

    let playlists = library.playlists().await.expect("failed to get playlists");
    process_playlists(playlists).await;
}

async fn process_photo_library(library: PhotoLibrary) {
    let albums = library.albums().await.expect("failed to get albums");
    println!("Received {} album(s)", albums.len());

    for album in albums {
        println!("Album '{}'", album.title());
        println!(
            "Received {} album item(s)",
            album
                .contents()
                .await
                .expect("failed to get album contents")
                .len()
        );
    }

    let playlists = library.playlists().await.expect("failed to get playlists");
    process_playlists(playlists).await;
}

#[async_std::main]
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

    for library in device.libraries() {
        println!(
            "{} (id={}, type={})",
            library.title(),
            library.id(),
            library.library_type()
        );

        match library {
            Library::Movie(library) | Library::Video(library) => {
                process_movie_library(library).await
            }
            Library::TV(library) => process_tv_library(library).await,
            Library::Music(library) => process_music_library(library).await,
            Library::Photo(library) => process_photo_library(library).await,
        }

        println!();
    }
}
