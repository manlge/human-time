use std::{
    thread::{self},
    time::Duration,
};

use human_time::ToHumanTimeString;

fn main() {
    println!(
        "costs {}",
        Duration::from_secs(88401 * 2 * 8).to_human_time_string()
    );
    println!(
        "costs {}",
        Duration::from_millis(8840003).to_human_time_string()
    );
    println!("costs {}", Duration::from_millis(0).to_human_time_string());
    println!(
        "costs {}",
        Duration::from_millis(8840003).to_human_time_string_with_format(
            |n, unit| {
                format!(
                    "{n}{}",
                    match unit {
                        "d" => "days".to_owned(),
                        "h" => "hours".to_owned(),
                        "m" => "minutes".to_owned(),
                        "s" => "seconds".to_owned(),
                        "ms" => "ms".to_owned(),
                        other => other.to_string(),
                    }
                )
            },
            |acc, item| format!("{} {}", acc, item)
        )
    );

    foo();
}

#[human_time::elapsed]
fn foo() {
    thread::sleep(Duration::from_millis(1000));
}
