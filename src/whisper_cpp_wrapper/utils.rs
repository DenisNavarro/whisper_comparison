#![allow(dead_code)]

use num::integer::div_floor;
use std::{fs::File, io::Write, path::PathBuf};

pub fn format_timestamp(seconds: i64, always_include_hours: bool, decimal_marker: &str) -> String {
    assert!(seconds >= 0, "non-negative timestamp expected");
    let mut milliseconds = seconds * 1000;

    let hours = div_floor(milliseconds, 3_600_000);
    milliseconds -= hours * 3_600_000;

    let minutes = div_floor(milliseconds, 60_000);
    milliseconds -= minutes * 60_000;

    let seconds = div_floor(milliseconds, 1_000);
    milliseconds -= seconds * 1_000;

    let hours_marker =
        if always_include_hours || hours != 0 { format!("{hours}:") } else { String::new() };

    format!("{hours_marker}{minutes:02}:{seconds:02}{decimal_marker}{milliseconds:03}")
}

pub fn write_to(path: PathBuf, content: &String) {
    File::create(path).unwrap().write_all(content.as_bytes()).unwrap();
}
