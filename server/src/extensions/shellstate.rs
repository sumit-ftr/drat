use std::path::{Path, PathBuf};

#[derive(Clone)]
pub struct ShellState {
    pub pwd: PathBuf,
    pub old_pwd: PathBuf,
    pub home_dir: PathBuf,
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
