use std::{
    thread,
    time::{Duration, Instant},
};

use human_time::ToHumanTimeString;

fn main() {
    let start = Instant::now();
    thread::sleep(Duration::from_secs(1));
    let costs: Duration = start.elapsed();
    println!("costs {}", costs.to_human_time_string());

    println!(
        "costs {}",
        Duration::from_secs(88401 * 2 * 8).to_human_time_string()
    );
    println!(
        "costs {}",
        Duration::from_millis(8840003).to_human_time_string()
    );
}
