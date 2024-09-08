use reqwest;
use serde::Deserialize;
use serde_json;

pub async fn fetch_sys_info() -> String {
    let mut response = String::with_capacity(1024);

    response.push_str(&sys_info::fetch_prompt());
    response.push_str(&get_location().await);
    response.push_str(&sys_info::fetch_os_info());
    response.push_str(&sys_info::fetch_kernel_info());
    response.push_str(&sys_info::fetch_shell_info());
    response.push_str(&sys_info::fetch_uptime());
    response.push_str(&sys_info::fetch_disk_usage());
    response.push_str(&sys_info::fetch_memory());
    response.push_str(&sys_info::fetch_cpu());
    response.push_str(&sys_info::fetch_resolution());

    response
}

#[allow(dead_code, non_snake_case)]
#[derive(Debug, Deserialize)]
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

pub async fn get_location() -> String {
    let mut ipv4 = String::new();
    while let Ok(r) = reqwest::get("https://api.ipify.org").await {
        ipv4 = r.text().await.unwrap();
        break;
    }
    while let Ok(r) = reqwest::get(format!("https://geolocation-db.com/json/{ipv4}")).await {
        let x = r.text().await.unwrap();
        let y = serde_json::from_str::<LocationResponse>(&x).unwrap();
        return format!(
            "Country: {} ({}) {}N {}W\nIPv4: {}\n",
            y.country_name.unwrap(),
            y.country_code.unwrap(),
            y.latitude,
            y.longitude,
            y.IPv4
        );
    }
    format!("")
}
