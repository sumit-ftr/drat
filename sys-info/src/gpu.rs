pub fn fetch_gpu() -> String {
    let lspci = std::process::Command::new("lspci")
        .arg("-mm")
        .output()
        .unwrap();

    let s = String::from_utf8(lspci.stdout).unwrap();

    format!("")
}
