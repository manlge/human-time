use std::time::Duration;

pub use human_time_macros::elapsed;

pub trait ToHumanTimeString {
    fn to_human_time_string(&self) -> String;
    fn to_human_time_string_with_format<F, F1>(&self, time_fmt: F, res_fmt: F1) -> String
    where
        F: Fn(u128, &str) -> String,
        F1: Fn(String, String) -> String;
}

impl ToHumanTimeString for Duration {
    fn to_human_time_string(&self) -> String {
        crate::human_time(*self)
    }

    fn to_human_time_string_with_format<F, F1>(&self, time_fmt: F, res_fmt: F1) -> String
    where
        F: Fn(u128, &str) -> String,
        F1: Fn(String, String) -> String,
    {
        human_time_with_format(*self, time_fmt, res_fmt)
    }
}

pub fn human_time(d: Duration) -> String {
    human_time_with_format(
        d,
        |n, unit| format!("{}{}", n, unit),
        |acc, item| format!("{},{}", acc, item),
    )
}

pub fn human_time_with_format<F, F1>(d: Duration, time_fmt: F, res_fmt: F1) -> String
where
    F: Fn(u128, &str) -> String,
    F1: Fn(String, String) -> String,
{
    let mut map: Vec<(u128, &str)> = Vec::new();
    let mut ms = d.as_millis();
    for (unit, n_ms) in [
        ("d", 86400000),
        ("h", 3600000),
        ("m", 60000),
        ("s", 1000),
        ("ms", 1),
    ] {
        map.push((ms / n_ms, unit));
        ms %= n_ms;
    }

    match map
        .into_iter()
        .filter_map(|(n, u)| if n > 0 { Some(time_fmt(n, u)) } else { None })
        .reduce(res_fmt)
    {
        Some(val) => val,
        None => time_fmt(0, "ms"),
    }
}
