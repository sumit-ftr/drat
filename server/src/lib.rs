// modules
pub mod extensions;
pub mod routes;

// imports
use crate::extensions::ShellState;
use axum;
use std::sync::{Arc, Mutex};
use tokio;

pub async fn run() {
    let shellstate = Arc::new(Mutex::new(ShellState::new()));
    let router = crate::routes::all_routes(Arc::clone(&shellstate));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .unwrap();

    println!("[-] listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, router).await.unwrap();
}
