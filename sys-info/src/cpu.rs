use std::path::PathBuf;

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
    let mut cores = 0u64;
    if let Ok(lscpu) = std::process::Command::new("lscpu").output() {
        for line in String::from_utf8(lscpu.stdout).unwrap().lines() {
            let mut it = line.split(':');
            if let (Some(key), Some(val)) = (it.next(), it.next()) {
                if key == "CPU(s)" {
                    cores = val.trim().parse().unwrap();
                    break;
                }
            }
        }
    }
    format!("CPU: {cpu_model} ({cores}) @ {freq}GHz\n")
}
