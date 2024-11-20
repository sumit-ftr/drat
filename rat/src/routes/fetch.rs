use axum::Json;
use reqwest;
use serde::Deserialize;
use serde_json;
use sysinfo::{Disks, System};

pub async fn fetch_sysinfo() -> Json<SysInfo> {
    let system = System::new_all();
    let cpu_list = system.cpus();
    let name_freq = cpu_list
        .iter()
        .map(|item| (item.name().to_string(), item.frequency()))
        .collect::<Vec<(String, u64)>>();
    let (country, ipv4) = get_location().await;

    // println!("{:?}", System::load_average());
    // println!("{:?}", system.cgroup_limits().unwrap());
    // println!("{}", system.process());
    // println!("{}", system.processes());

    // response.push_str(&sys_info::fetch_shell_info());
    // response.push_str(&sys_info::fetch_resolution());
    // battery
    // gpu

    Json(SysInfo {
        prompt: format!(
            "{}@{}",
            std::env::var("USER").unwrap(),
            System::host_name().unwrap()
        ),
        country,
        IPv4: ipv4,
        os: System::long_os_version().unwrap(), // ignored: name, distribution_id, os_version
        kernel: System::kernel_version().unwrap(),
        uptime: formatted_time(System::uptime()),
        memory: MemInfo {
            total: formatted_memory(system.total_memory()),
            used: formatted_memory(system.used_memory()),
            avail: formatted_memory(system.available_memory()),
            free: formatted_memory(system.free_memory()),
        },
        cpu: CpuInfo {
            arch: System::cpu_arch().unwrap(),
            usage: system.global_cpu_usage(),
            cores: system.physical_core_count().unwrap(),
            cpus: name_freq.len(),
            name_freq,
            vendor_id: cpu_list.iter().next().unwrap().vendor_id().to_string(),
            brand: cpu_list.iter().next().unwrap().brand().to_string(),
        },
        disk: Disks::new_with_refreshed_list()
            .list()
            .into_iter()
            .map(|item| DiskInfo {
                avail: formatted_memory(item.available_space()),
                total: formatted_memory(item.total_space()),
                filesystem: item.file_system().to_str().unwrap().to_string(),
                name: item.name().to_str().unwrap().to_string(),
                kind: item.kind().to_string(),
                is_removable: item.is_removable(),
                mount_point: item.mount_point().to_str().unwrap().to_string(),
            })
            .collect::<Vec<DiskInfo>>(),
        swap: SwapInfo {
            total: formatted_memory(system.total_swap()),
            used: formatted_memory(system.used_swap()),
            free: formatted_memory(system.free_swap()),
        },
        boot_time: formatted_time(System::boot_time()),
    })
}

#[allow(non_snake_case)]
#[derive(serde::Serialize)]
pub struct SysInfo {
    prompt: String,
    country: String,
    IPv4: String,
    os: String,
    kernel: String,
    uptime: String,
    memory: MemInfo,
    cpu: CpuInfo,
    disk: Vec<DiskInfo>,
    swap: SwapInfo,
    boot_time: String,
}

#[derive(serde::Serialize)]
struct MemInfo {
    total: String,
    used: String,
    avail: String,
    free: String,
}

#[derive(serde::Serialize)]
struct CpuInfo {
    arch: String,
    usage: f32,
    cores: usize,
    cpus: usize,
    name_freq: Vec<(String, u64)>, // Vec<(&'a str, u64)>
    vendor_id: String,             // &'a str,
    brand: String,                 // &'a str,
}

#[derive(serde::Serialize)]
struct SwapInfo {
    total: String,
    used: String,
    free: String,
}

#[derive(serde::Serialize)]
struct DiskInfo {
    total: String,
    avail: String,
    filesystem: String, // &'a OsStr
    name: String,       // &'a OsStr
    kind: String,
    is_removable: bool,
    mount_point: String, // &'a Path
}

pub(crate) fn formatted_memory(total_bytes: u64) -> String {
    let (gib, rem_bytes) = (total_bytes / 1073741824, total_bytes % 1073741824);
    let (mib, rem_bytes) = (rem_bytes / 1048576, rem_bytes % 1048576);
    let (kib, bytes) = (rem_bytes / 1024, rem_bytes % 1024);

    format!("{gib}G {mib}M {kib}K {bytes}B")
}

pub(crate) fn formatted_time(seconds: u64) -> String {
    let (days, rem_secs) = (seconds / 86400, seconds % 86400);
    let (hours, rem_secs) = (rem_secs / 3600, rem_secs % 3600);
    let (mins, secs) = (rem_secs / 60, rem_secs % 60);

    format!("{days}d {hours}h {mins}m {secs}s")
}

#[allow(dead_code, non_snake_case)]
#[derive(Deserialize)]
struct LocationResponse {
    country_code: Option<String>,
    country_name: Option<String>,
    city: Option<String>,
    postal: Option<String>,
    latitude: f64,
    longitude: f64,
    IPv4: String,
    state: Option<String>,
}

pub async fn get_location() -> (String, String) {
    let mut ipv4 = String::new();
    while let Ok(r) = reqwest::get("https://api.ipify.org").await {
        ipv4 = r.text().await.unwrap();
        break;
    }
    while let Ok(r) = reqwest::get(format!("https://geolocation-db.com/json/{ipv4}")).await {
        let x = r.text().await.unwrap();
        let y = serde_json::from_str::<LocationResponse>(&x).unwrap();
        return (
            format!(
                "{} ({}) {}N {}W",
                y.country_name.unwrap(),
                y.country_code.unwrap(),
                y.latitude,
                y.longitude,
            ),
            format!("{ipv4}"),
        );
    }
    (format!(""), format!(""))
}
