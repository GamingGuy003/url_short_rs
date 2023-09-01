use std::net::SocketAddr;

use axum::{extract::ConnectInfo, Router, Extension};
use axum_sqlite::Database;

static DBURL: &str = "db";
static TABLE: &str = "translations";

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // create server addr
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));

    // define routes
    let app = Router::new()
        .route("/:uri", axum::routing::get(follow))
        .route("/_shorten/:uri", axum::routing::post(shorten))
        .route("/_delete/:uri", axum::routing::delete(delete))
        .route("/_info/:uri", axum::routing::get(info))
        .layer(Database::new(DBURL).expect("Could not connect to sqlite"));

    // startup webserver
    axum::Server::bind(&addr)
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();

    // big brain
    Ok(())
}


async fn follow(ConnectInfo(connection): ConnectInfo<SocketAddr>, Extension(database): Extension<Database>) {
    self::db_prepare(Extension(database));
}

async fn shorten(ConnectInfo(connection): ConnectInfo<SocketAddr>, Extension(database): Extension<Database>) {
    self::db_prepare(Extension(database));
}

async fn delete(ConnectInfo(connection): ConnectInfo<SocketAddr>, Extension(database): Extension<Database>) {
    self::db_prepare(Extension(database));
}

async fn info(ConnectInfo(connection): ConnectInfo<SocketAddr>, Extension(database): Extension<Database>) {
    self::db_prepare(Extension(database));
}

fn db_prepare(Extension(database): Extension<Database>) {
    let connection = database.connection().expect("Could not establish db connection");
    let query = format!("SELECT name FROM sqlite_master WHERE type='table' AND name='{}'", TABLE);
    let exists = connection.query_row(&query, [], |row| { Ok(row.get::<_, Option<String>>(0)?.is_some()) }).is_ok();
    println!("exists: {}", exists);
    if !exists {
        println!("Database does not exist, creating...");
        let query = format!("CREATE TABLE {} ( original LONGTEXT, shortened TEXT );", TABLE);
        connection.execute(&query, []).expect("Failed to create table");
    }
}