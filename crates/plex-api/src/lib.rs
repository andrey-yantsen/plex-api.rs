//! TODO
//!
mod client;
mod error;
pub(crate) mod media_container;
mod myplex;
mod server;
pub mod url;

pub use client::ClientBuilder;
pub use error::Error;
pub use myplex::MyPlexBuilder;
pub use server::Server;

pub type Result<T = (), E = error::Error> = std::result::Result<T, E>;
