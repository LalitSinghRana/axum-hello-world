use axum::{http::{ Method }, routing::get, Router};
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {
    println!("@main : started");
    // build our application with a route
    let app = Router::new().route("/", get(handler)).layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods([Method::GET]),
        );

    // run it
    let addr = "[::]:3030".parse::<SocketAddr>().unwrap();
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handler() -> &'static str {
    "Hello, World!"
}
