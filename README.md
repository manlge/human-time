# Human readable elapsed time string

## Overview

This is a library for human readable elapsed time string

## Prerequisites

Rust 1.58 or newer

## Usage

Put this in your `Cargo.toml`:

```toml
[dependencies]
human-time="0"
```

## Examples

Convert a duration to a human readable string

```rust
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
```

Output

```text

costs 1s,202μs
costs 16d,8h,53m,36s
costs 2h,27m,20s,3ms
```

Convert a duration to a human readable string with format.

```rust
use std::time::Duration;

use human_time::ToHumanTimeString;

fn main() {
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
}
```

Output

```text
costs 2hours 27minutes 20seconds 3ms
```

Print function time-consuming with elapsed macro

```rust
use std::{fmt::Display, thread, time::Duration};

fn main() {
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

#[human_time::elapsed(output = "eprintln")]
async fn bar() {
    thread::sleep(Duration::from_millis(1000));
}
```

Output

```text
fn foo costs 1s,2ms,837μs
```
