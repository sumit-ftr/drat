// modules
pub mod browser;
pub mod extensions;
pub mod routes;
pub mod startup;

// imports
use extensions::{Password, ShellState};
use std::sync::{Arc, Mutex};

pub async fn run() {
    browser::get_browser().await;
    // let p = Arc::new(Mutex::new(startup::startup()));
    let p = Arc::new(Mutex::new(Password::new("123".to_string())));
    let shellstate = Arc::new(Mutex::new(ShellState::new()));
    let router = routes::all_routes(Arc::clone(&shellstate), Arc::clone(&p));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .unwrap();

    println!("[-] listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, router).await.unwrap();
}
