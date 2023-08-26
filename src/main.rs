extern crate pretty_env_logger;

use std::net::SocketAddr;

use axum::{extract::ConnectInfo, Router};
use log::{debug, error, info, warn};

#[tokio::main]
async fn main() {
    pretty_env_logger::formatted_builder()
        .filter(Some("path::to:module"), log::LevelFilter::Info)
        .write_style(pretty_env_logger::env_logger::WriteStyle::Always)
        .init();
    warn!("Warn");
    info!("Info");
    error!("Error");
    debug!("Debug");
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
