pub async fn fetch_sys_info() -> String {
    let mut response = String::with_capacity(1024);

    response.push_str(&sys_info::fetch_prompt());
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
