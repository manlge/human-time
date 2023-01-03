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
