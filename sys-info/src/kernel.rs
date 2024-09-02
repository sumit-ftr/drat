pub fn fetch_kernel_info() -> String {
    let kernel_type = std::fs::read_to_string("/proc/sys/kernel/ostype").unwrap();
    let kernel_release = std::fs::read_to_string("/proc/sys/kernel/osrelease").unwrap();

    format!(
        "Kernel: {} {}",
        &kernel_type[..kernel_type.len() - 1],
        kernel_release
    )
}
