pub fn fetch_kernel_info() -> String {
    let kernel = std::process::Command::new("uname")
        .arg("-sr")
        .output()
        .unwrap();

    format!("Kernel: {}", String::from_utf8(kernel.stdout).unwrap())
}
