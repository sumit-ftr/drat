pub fn fetch_shell_info() -> String {
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
