use sysinfo::System;

// Calculate bytes to gigabytes
fn bytes_to_gib(bytes: u64) -> f64 {
    return bytes as f64 / 1024_f64.powi(3);
}

pub fn systeminfo() -> String {
    let mut sys = System::new_all();
    sys.refresh_all();

    let name = System::name();
    let os = System::os_version();
    let kernel = System::kernel_version();

    format!(
        "\n-----------------------------------------\nos    {:?} {:?}\nker   {:?}\nmem   {:.2}GiB\nswap  {:.2}GiB",
        name,
        os,
        kernel,
        bytes_to_gib(sys.total_memory()),
        bytes_to_gib(sys.total_swap()),
    )
}
