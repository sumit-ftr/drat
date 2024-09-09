// modules
pub mod cookie;
pub mod extensions;
pub mod routes;
pub mod startup;

// imports
use axum;
use extensions::ShellState;
use std::sync::{Arc, Mutex};
use tokio;

pub async fn run() {
    let p = Arc::new(Mutex::new(startup::startup()));
    let shellstate = Arc::new(Mutex::new(ShellState::new()));
    let router = routes::all_routes(Arc::clone(&shellstate), Arc::clone(&p));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .unwrap();

    println!("[-] listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, router).await.unwrap();
}
