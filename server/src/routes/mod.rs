mod exec;

use axum::{
    routing::{get, post},
    Extension, Router,
};
use std::path::PathBuf;

#[derive(Clone)]
pub struct ShellPath {
    pwd: PathBuf,
    old_pwd: PathBuf,
    home_dir: PathBuf,
}

pub fn all_routes() -> Router {
    let common_dir = std::env::var("HOME").unwrap();

    Router::new()
        .route("/", get("login with username and password"))
        .route("/set", get("update username & password"))
        .route("/exec", post(exec::exec_cmd))
        .layer(Extension(ShellPath {
            pwd: PathBuf::from(common_dir.clone()),
            old_pwd: PathBuf::from(common_dir.clone()),
            home_dir: PathBuf::from(common_dir),
        }))
}
