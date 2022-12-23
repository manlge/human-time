# Human readable time string

## Overview

This is a library for human readable elapsed time string

## Prerequisites

Rust 1.6 or newer

## Usage

Put this in your `Cargo.toml`:

```toml
[dependencies]
human-time="0"
```

## Example

```rust
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
            |acc, item| format!("{}, {}", acc, item)
        )
    );

    foo();
}

#[human_time::elapsed]
fn foo() {
    thread::sleep(Duration::from_millis(100));
}
```
Output
```text
costs 16d,8h,53m,36s
costs 2h,27m,20s,3ms
costs 0ms
costs 2hours, 27minutes, 20seconds, 3ms
fn foo costs 1s,3ms
```