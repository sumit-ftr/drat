pub fn fetch_uptime() -> String {
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
