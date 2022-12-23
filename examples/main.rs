use std::{
    thread::{self},
    time::Duration,
};

fn main() {
    let start = std::time::Instant::now();
    thread::sleep(Duration::from_secs(1));
    println!("costs {}", human_time::human_time(start.elapsed()));

    foo();
}

#[human_time::elapsed]
fn foo() -> i32 {
    thread::sleep(Duration::from_millis(100));
    1_i32
}
