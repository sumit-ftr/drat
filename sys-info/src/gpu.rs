pub fn fetch_gpu() -> String {
    if let Ok(lspci) = std::process::Command::new("lspci").arg("-mm").output() {
        let s = String::from_utf8(lspci.stdout).unwrap();
        for line in s.lines() {
            return format!("{line}");
        }
    }

    format!("")
}
