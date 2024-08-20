// modules
pub mod routes;

// imports
use axum;
use tokio;

// Common Server Error type
pub type ServerError<T> = Result<T, Box<dyn std::error::Error>>;

pub async fn run() {
    let router = crate::routes::all_routes();

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .unwrap();

    println!("[-] listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, router).await.unwrap();
}
