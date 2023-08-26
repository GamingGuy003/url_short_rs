use std::{net::SocketAddr, sync::{Arc, Mutex}};

use axum::{extract::ConnectInfo, Router};
use logger::{logger::Logger, level::Level::Info, log_field::LogField::{Date, Time, Seperator, Message, Level}};


static LOGGER: Arc<Mutex<Logger>> = Arc::new(Mutex::new(Logger::new(Info, std::io::stdout(), vec![Date, Seperator(String::from("T")), Time, Seperator(String::from(" ")), Level, Seperator(String::from("")), Message])));

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    let app = Router::new()
        .route("/:uri", axum::routing::get(follow))
        .route("/_shorten/:uri", axum::routing::post(shorten))
        .route("/_delete/:uri", axum::routing::delete(delete))
        .route("/_info/:uri", axum::routing::get(info));
    axum::Server::bind(&addr)
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();
}

async fn follow(ConnectInfo(connection): ConnectInfo<SocketAddr>) {}
async fn shorten(ConnectInfo(connection): ConnectInfo<SocketAddr>) {}
async fn delete(ConnectInfo(connection): ConnectInfo<SocketAddr>) {}
async fn info(ConnectInfo(connection): ConnectInfo<SocketAddr>) {}
