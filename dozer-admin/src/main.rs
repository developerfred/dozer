extern crate diesel;
pub mod db;
pub mod server;
pub mod services;
pub mod utils;

#[tokio::main]
async fn main() {
    server::get_server().await.unwrap();
}