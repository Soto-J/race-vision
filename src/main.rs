use std::process;
use sysinfo::System;

fn main() {
    let mut sys = System::new_all();
    sys.refresh_all();

    for (pid, process) in sys.processes() {
        println!(
            "PID: {pid} | Name: {:?} |  Memory: {} KB",
            process.name(),
            process.memory() / 1024
        )
    }

    let pid = process::id();

    println!("Current process PID: {}", pid);
}
