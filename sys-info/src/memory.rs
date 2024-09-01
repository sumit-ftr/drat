pub fn fetch_memory() -> String {
    let meminfo = std::fs::read_to_string("/proc/meminfo").unwrap();
    let lines = meminfo.lines();
    let mut mem_total = 0u64;
    let mut mem_used = 0u64;

    for line in lines.into_iter() {
        let mut it = line.split_ascii_whitespace();
        let key = it.next().unwrap();
        let val = it.next().unwrap().parse::<u64>().unwrap();

        if key == "MemTotal:" {
            mem_total += val;
            mem_used += val;
        } else if key == "Shmem:" {
            mem_used += val;
        } else if key == "MemFree:" {
            mem_used -= val;
        } else if key == "Buffers:" {
            mem_used -= val;
        } else if key == "Cached:" {
            mem_used -= val;
        } else if key == "SReclaimable:" {
            mem_used -= val;
        }
    }

    format!(
        "MemTotal: {}\nMemUsed: {}\nMemAvail: {}\n",
        formatted_memory(mem_total),
        formatted_memory(mem_used),
        formatted_memory(mem_total - mem_used)
    )
}

fn formatted_memory(kb: u64) -> String {
    let total_bytes = 1000 * kb;

    let (gib, rem_bytes) = (total_bytes / 1073741824, total_bytes % 1073741824);
    let (mib, rem_bytes) = (rem_bytes / 1048576, rem_bytes % 1048576);
    let (kib, bytes) = (rem_bytes / 1024, rem_bytes % 1024);

    format!("{gib}G {mib}M {kib}K {bytes}B")
}
