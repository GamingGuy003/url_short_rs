use std::{net::SocketAddr, sync::{Arc, Mutex}};

use axum::{extract::ConnectInfo, Router};
use sqlite::Connection;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // create server addr
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));

    // connect to sqlite
    let sql_connection = Arc::new(Mutex::new(match sqlite::open("./db") {
        Ok(connection) => connection,
        Err(err) => return Err(std::io::Error::new(std::io::ErrorKind::NotConnected, err))
    }));

    // define routes
    let app = Router::new()
        .route("/:uri", axum::routing::get(follow))
        .route("/_shorten/:uri", axum::routing::post(shorten))
        .route("/_delete/:uri", axum::routing::delete(delete))
        .route("/_info/:uri", axum::routing::get(info));

    // startup webserver
    axum::Server::bind(&addr)
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();

    Ok(())
}

async fn initialize_db(mut connection: Connection) -> std::io::Result<()> {
    let query = "
        CREATE TABLE links(
            original LONGTEXT,
            shortened TEXT,
        );
    ";
    connection.execute(query).unwrap();
    Ok(())
}

async fn follow(ConnectInfo(connection): ConnectInfo<SocketAddr>) {

}

async fn shorten(ConnectInfo(connection): ConnectInfo<SocketAddr>) {

}

async fn delete(ConnectInfo(connection): ConnectInfo<SocketAddr>) {

}

async fn info(ConnectInfo(connection): ConnectInfo<SocketAddr>) {

}
