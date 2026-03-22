use std::{thread, time::Duration};

use std::collections::HashMap;
use sysinfo::{Networks, System};

struct NetworkState {
    prev_in: u64,
    prev_out: u64,
}

fn bytes_to_gb(bytes: u64) -> f64 {
    bytes as f64 / 1024.0f64.powi(3)
}

fn main() {
    println!("--- Starting sys monitor ---");

    let mut sys = System::new_all();
    let mut net = Networks::new_with_refreshed_list();
    let mut net_history: HashMap<String, NetworkState> = HashMap::new();

    for (name, data) in &net {
        net_history.insert(
            name.clone(),
            NetworkState {
                prev_in: data.received(),
                prev_out: data.transmitted(),
            },
        );
    }

    loop {
        sys.refresh_all();
        net.refresh(false);

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

        // Network
        println!("\nNetworking");
        for (interface_name, data) in &net {
            let curr_in = data.received();
            let curr_out = data.transmitted();

            let stats = net_history
                .entry(interface_name.clone())
                .or_insert(NetworkState {
                    prev_in: curr_in,
                    prev_out: curr_out,
                });

            let diff_in = curr_in.saturating_sub(stats.prev_in);
            let diff_out = curr_out.saturating_sub(stats.prev_out);

            stats.prev_in = curr_in;
            stats.prev_out = curr_out;

            println!(
                "  {}: ⬇️  {:.2} KB/s | ⬆️  {:.2} KB/s",
                interface_name,
                diff_in as f64 / 1024.0,
                diff_out as f64 / 1024.0
            );
        }

        thread::sleep(Duration::from_secs(1))
    }
}
