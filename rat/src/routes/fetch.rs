// os: String,
// resolution: String,
// cpu: String,
// gpu: String,
// memory: String,

use std::path::PathBuf;

pub async fn fetch_sys_info() -> String {
    let mut response = String::with_capacity(1024);

    response.push_str(&fetch_prompt());
    response.push_str(&fetch_kernel_info());
    response.push_str(&fetch_shell_info());
    response.push_str(&fetch_uptime());
    response.push_str(&fetch_memory());
    response.push_str(&fetch_cpu());

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

fn fetch_uptime() -> String {
    let uptime = std::fs::read_to_string("/proc/uptime").unwrap();
    let uptime_secs = uptime
        .split_ascii_whitespace()
        .next()
        .unwrap()
        .parse::<f64>()
        .unwrap() as u64;

    let (days, rem_secs) = (uptime_secs / 86400, uptime_secs % 86400);
    let (hours, rem_secs) = (rem_secs / 3600, rem_secs % 3600);
    let (mins, secs) = (rem_secs / 60, rem_secs % 60);

    format!("Uptime: {days}d {hours}h {mins}m {secs}s\n")
}

pub fn fetch_memory() -> String {
    let meminfo = std::fs::read_to_string("/proc/meminfo").unwrap();
    let lines = meminfo.lines();
    let mut mem_total = 0u64;
    let mut mem_used = 0u64;

    for line in lines.into_iter() {
        let mut it = line.split_ascii_whitespace();
        let key = it.next().unwrap();
        let val = it.next().unwrap().parse::<u64>().unwrap();

        if key == "MemTotal:" {
            mem_total += val;
            mem_used += val;
        } else if key == "Shmem:" {
            mem_used += val;
        } else if key == "MemFree:" {
            mem_used -= val;
        } else if key == "Buffers:" {
            mem_used -= val;
        } else if key == "Cached:" {
            mem_used -= val;
        } else if key == "SReclaimable:" {
            mem_used -= val;
        }
    }

    format!(
        "MemTotal: {}\nMemUsed: {}\nMemAvail: {}\n",
        formatted_memory(mem_total),
        formatted_memory(mem_used),
        formatted_memory(mem_total - mem_used)
    )
}

fn formatted_memory(kb: u64) -> String {
    let total_bytes = 1000 * kb;

    let (gib, rem_bytes) = (total_bytes / 1073741824, total_bytes % 1073741824);
    let (mib, rem_bytes) = (rem_bytes / 1048576, rem_bytes % 1048576);
    let (kib, bytes) = (rem_bytes / 1024, rem_bytes % 1024);

    format!("{gib}G {mib}M {kib}K {bytes}B")
}

pub fn fetch_cpu() -> String {
    let cpuinfo = std::fs::read_to_string("/proc/cpuinfo").unwrap();
    let mut cpu_model = "";

    // fetching cpu model name
    for line in cpuinfo.lines() {
        let mut it = line.split(':');
        if let (Some(mut key), Some(mut val)) = (it.next(), it.next()) {
            key = key.trim();
            val = val.trim();
            if key == "model name"
                || key == "Hardware"
                || key == "Processor"
                || key == "cpu model"
                || key == "chip type"
                || key == "cpu type"
            {
                cpu_model = val;
                break;
            }
        }
    }

    // fetching cpu frequency / clock speed
    let s = "/sys/devices/system/cpu/cpu0/cpufreq";
    let mut freq = 0f64;
    for i in ["bios_limit", "scaling_max_freq", "cpuinfo_max_freq"].iter() {
        let p = PathBuf::from(format!("{s}/{i}"));
        if let Ok(v) = std::fs::read_to_string(p) {
            freq = v[..v.len() - 1].parse::<f64>().unwrap() / 1000000.0;
            break;
        }
    }

    // fetching cpu cores
    let lscpu = std::process::Command::new("lscpu").output().unwrap();
    let mut cores = 0u64;
    for line in String::from_utf8(lscpu.stdout).unwrap().lines() {
        let mut it = line.split(':');
        if let (Some(key), Some(val)) = (it.next(), it.next()) {
            if key == "CPU(s)" {
                cores = val.trim().parse().unwrap();
                break;
            }
        }
    }

    format!("CPU: {cpu_model} ({cores}) @ {freq}GHz\n")
}

// pub fn fetch_os_info() -> String {}
// pub fn fetch_resolution() -> String {}
// pub fn fetch_gpu() -> String {}
// pub fn fetch_battery() -> String {}
