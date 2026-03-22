use std::{thread, time::Duration};

use sysinfo::System;

fn bytes_to_gb(bytes: u64) -> f64 {
    bytes as f64 / 1024.0f64.powi(3)
}

fn main() {
    println!("--- Starting sys monitor ---");

    let mut sys = System::new_all();

    loop {
        sys.refresh_all();

        print!("{}[2J", 27 as char);

        // Memory
        let total_mem = bytes_to_gb(sys.total_memory());
        let used_mem = bytes_to_gb(sys.used_memory());

        let mem_perc = (used_mem / total_mem) * 100.0;
        println!(
            "Memory {:.2} GB / {} GB {:.2}%",
            used_mem, total_mem, mem_perc
        );

        // CPU
        println!("\nCPUs");
        for (i, cpu) in sys.cpus().iter().enumerate() {
            println!("   CPU {}: {:.2}%", i, cpu.cpu_usage());
        }

        thread::sleep(Duration::from_secs(1))
    }
}
