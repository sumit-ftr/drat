pub fn fetch_resolution() -> String {
    if let Ok(r) = std::process::Command::new("xrandr")
        .args(["--nograb", "--current"])
        .output()
    {
        let s = String::from_utf8(r.stdout).unwrap();
        let mut s = s.lines();
        s.next();
        s.next();

        for line in s {
            let mut it = line.split_ascii_whitespace();
            if let (Some(key), Some(val)) = (it.next(), it.next()) {
                if val.contains("*+") {
                    return format!("Resolution: {key} @ {}Hz\n", &val[..val.len() - 2]);
                }
            }
        }
    }

    format!("")
}
