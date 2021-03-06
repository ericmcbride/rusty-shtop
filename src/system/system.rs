extern crate num_cpus;
extern crate sys_info;

#[derive(Debug)]
pub struct SystemInformation {
    num_cpus: usize,
    processor_speed: u64,
    os_type: String,
    os_release: String,
    disk_info_free: u64,
    disk_info_total: u64,
    hostname: String,
}

impl SystemInformation {
    pub fn new() -> SystemInformation {
        SystemInformation {
            num_cpus: get_cpus(),
            processor_speed: get_cpu_speed(),
            os_type: get_os_type(),
            os_release: get_os_release(),
            disk_info_free: get_disk_info_free(),
            disk_info_total: get_disk_info_total(),
            hostname: get_hostname(),
        }
    }
}
pub fn get_cpus() -> usize {
    return num_cpus::get();
}

pub fn get_cpu_speed() -> u64 {
    return sys_info::cpu_speed().unwrap();
}

pub fn get_os_type() -> String {
    return sys_info::os_type().unwrap();
}

pub fn get_os_release() -> String {
    return sys_info::os_type().unwrap();
}

pub fn get_system_load_average() -> sys_info::LoadAvg {
    return sys_info::loadavg().unwrap();
}

pub fn get_disk_info_free() -> u64 {
    let disk_free = sys_info::disk_info().unwrap();
    return disk_free.free;
}

pub fn get_disk_info_total() -> u64 {
    let disk_total = sys_info::disk_info().unwrap();
    return disk_total.total;
}
pub fn get_hostname() -> String {
    return sys_info::hostname().unwrap();
}
