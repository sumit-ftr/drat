mod cpu;
mod kernel;
mod memory;
mod os;
mod prompt;
mod shell;
mod storage;
mod uptime;

// remaining
mod battery;
mod gpu;
mod resolution;

pub use cpu::fetch_cpu;
pub use gpu::fetch_gpu;
pub use kernel::fetch_kernel_info;
pub use memory::fetch_memory;
pub use os::fetch_os_info;
pub use prompt::fetch_prompt;
pub use shell::fetch_shell_info;
pub use storage::fetch_disk_usage;
pub use uptime::fetch_uptime;

pub(crate) fn formatted_memory(kb: u64) -> String {
    let total_bytes = 1000 * kb;

    let (gib, rem_bytes) = (total_bytes / 1073741824, total_bytes % 1073741824);
    let (mib, rem_bytes) = (rem_bytes / 1048576, rem_bytes % 1048576);
    let (kib, bytes) = (rem_bytes / 1024, rem_bytes % 1024);

    format!("{gib}G {mib}M {kib}K {bytes}B")
}
