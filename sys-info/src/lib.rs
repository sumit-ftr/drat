mod battery;
mod distro;
mod gpu;
mod resolution;
mod storage;

// done
mod cpu;
mod kernel;
mod memory;
mod prompt;
mod shell;
mod uptime;

pub use cpu::fetch_cpu;
pub use kernel::fetch_kernel_info;
pub use memory::fetch_memory;
pub use prompt::fetch_prompt;
pub use shell::fetch_shell_info;
pub use uptime::fetch_uptime;
