pub fn fetch_os_info() -> String {
    let file = std::fs::read_to_string("/etc/os-release").unwrap();
    let mut name = "";
    let mut version = "";
    for line in file.lines() {
        let mut it = line.split('=');
        if let (Some(key), Some(mut val)) = (it.next(), it.next()) {
            if val.starts_with('"') && val.ends_with('"') {
                val = &val[1..val.len() - 1];
            }

            if key == "NAME" {
                name = val;
            } else if key == "VERSION" {
                version = val;
            }
        }
    }
    format!("OS: {name} {version}\n")
}
