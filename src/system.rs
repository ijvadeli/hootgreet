use sysinfo::System;

pub fn systeminfo() -> String {
    let mut sys = System::new_all();
    sys.refresh_all();

    let name = System::name();
    let os = System::os_version();
    let kernel = System::kernel_version();

    format!(
        "\nos:   {:?} {:?}\nker:  {:?}\nmem:  {}\nswap: {}",
        name,
        os,
        kernel,
        sys.used_memory(),
        sys.total_swap()
    )
}
