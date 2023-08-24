use std::net::SocketAddr;

use axum::{extract::ConnectInfo, routing::get, Router};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(test));
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    axum::Server::bind(&addr)
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();
}

async fn test(ConnectInfo(addr): ConnectInfo<SocketAddr>) -> String {
    println!("Handled get to / from {:#}", addr);
    String::new()
}
