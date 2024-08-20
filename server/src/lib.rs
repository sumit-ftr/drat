// modules
pub mod routes;

// imports
use axum;
use std::{
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
};
use tokio;

#[derive(Clone)]
pub struct ShellState {
    pwd: PathBuf,
    old_pwd: PathBuf,
    home_dir: PathBuf,
}

impl ShellState {
    pub fn new() -> Self {
        let home_dir = std::env::var("HOME").unwrap();
        Self {
            pwd: PathBuf::from(home_dir.clone()),
            old_pwd: PathBuf::from(home_dir.clone()),
            home_dir: PathBuf::from(home_dir),
        }
    }

    pub fn manage_path(&mut self, cmd_rest: Vec<&str>) {
        if cmd_rest.len() == 0 || cmd_rest[0] == "~" {
            let p = std::mem::replace(&mut self.pwd, self.home_dir.clone());
            let _ = std::mem::replace(&mut self.old_pwd, p);
        } else if cmd_rest[0] == ".." {
            self.old_pwd = self.pwd.clone();
            self.pwd.pop();
        } else if cmd_rest[0] == "-" {
            std::mem::swap(&mut self.old_pwd, &mut self.pwd);
        } else {
            let new_path = Box::new(Path::new(cmd_rest[0]));
            if new_path.is_relative() {
                self.old_pwd = self.pwd.clone();
                self.pwd.push(*new_path);
            } else if new_path.is_absolute() {
                let p1 = PathBuf::from(*new_path);
                let p2 = std::mem::replace(&mut self.pwd, p1);
                let _ = std::mem::replace(&mut self.old_pwd, p2);
            }
        }
    }
}

pub async fn run() {
    let shellstate = Arc::new(Mutex::new(ShellState::new()));
    let router = crate::routes::all_routes(Arc::clone(&shellstate));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .unwrap();

    println!("[-] listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, router).await.unwrap();
}
