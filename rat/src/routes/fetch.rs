// os: String,
// resolution: String,
// cpu: String,
// gpu: String,
// memory: String,

pub async fn fetch_sys_info() -> String {
    let mut response = String::with_capacity(1024);

    response.push_str(&fetch_prompt());
    response.push_str(&fetch_kernel_info());
    response.push_str(&fetch_shell_info());
    response.push_str(&fetch_uptime().await);

    response
}

fn fetch_prompt() -> String {
    let user = std::env::var("USER").unwrap();
    let hostname = std::env::var("HOSTNAME").unwrap();
    format!("{user}@{hostname}\n")
}

fn fetch_kernel_info() -> String {
    let kernel = std::process::Command::new("uname")
        .arg("-sr")
        .output()
        .unwrap();

    format!("Kernel: {}", String::from_utf8(kernel.stdout).unwrap())
}

fn fetch_shell_info() -> String {
    let shell = std::env::var("SHELL").unwrap();
    let mut version = String::from_utf8(
        std::process::Command::new(&shell)
            .arg("--version")
            .output()
            .unwrap()
            .stdout,
    )
    .unwrap();
    version.pop();

    format!("Shell: {version} (Path: {shell})\n")
}

async fn fetch_uptime() -> String {
    let uptime = tokio::fs::read_to_string("/proc/uptime").await.unwrap();
    let uptime_secs = uptime
        .split_ascii_whitespace()
        .next()
        .unwrap()
        .parse::<f64>()
        .unwrap() as u64;

    let (days, rem_secs) = (uptime_secs / 86400, uptime_secs % 86400);
    let (hours, rem_secs) = (rem_secs / 3600, rem_secs % 3600);
    let (mins, secs) = (rem_secs / 60, rem_secs % 60);

    format!("Uptime: {days}d {hours}h {mins}m {secs}s")
}

// pub fn fetch_os_info() -> String {}
// pub fn fetch_resolution() -> String {}
// pub fn fetch_cpu() -> String {}
// pub fn fetch_gpu() -> String {}
// pub fn fetch_memory() -> String {}
