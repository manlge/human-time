use std::{
    fmt::Display,
    thread::{self},
    time::{Duration, Instant},
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

    let start = Instant::now();
    thread::sleep(Duration::from_secs(1));
    println!("costs {}", start.elapsed().to_human_time_string());
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

    foo(1);
}

//use log::debug;
//#[human_time::elapsed(output = "debug")]
//#[human_time::elapsed(output = "eprintln")]
// #[human_time::elapsed(output = "println")] //default
#[human_time::elapsed()]
fn foo<T>(_x: T)
where
    T: Display,
{
    thread::sleep(Duration::from_millis(1000));
}

#[human_time::elapsed()]
async fn bar() {
    thread::sleep(Duration::from_millis(1000));
}
