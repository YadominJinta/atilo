use uname::uname;

pub fn get_sysinfo() {
    let info = uname().unwrap();
    println!("System:");
    println!("  Images: {}", 0);
    println!("  Hostname: {}", info.nodename);
    println!("  Kernel Version: {}", info.release);
    println!("  CPU Arch: {}", info.machine);
}