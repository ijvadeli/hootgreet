use sysinfo::System;

pub fn systeminfo() {
    let mut sys = System::new_all();
    sys.refresh_all();

    let name = System::name();
    let os = System::os_version();
    let kernel = System::kernel_version();

    println!("os  : {:?} {:?}", name, os);
    println!("ker : {:?}", kernel);
    println!("mem : {}", sys.used_memory());
    println!("swap: {}", sys.total_swap());
}
