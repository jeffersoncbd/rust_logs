use crate::time;
use chrono::FixedOffset;
use std::{
    fs::{self, File, OpenOptions},
    io::{ErrorKind, Write},
    path::Path,
};

pub fn write(log: &String, time_zone: Option<&FixedOffset>) {
    let file_name = format!("logs/{}.txt", time::now(time_zone).get_date());

    if !Path::new("logs").exists() {
        fs::create_dir("logs").expect("Unable to create logs directory.")
    }

    if let Err(error) = fs::metadata(&file_name) {
        match error.kind() {
            ErrorKind::NotFound => {
                File::create(&file_name).expect("Unable to create logs file");
            }
            _ => panic!("{error}"),
        }
    }

    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(file_name)
        .unwrap();
    let log = log.replace("  ", " ");
    writeln!(file, "{log}").unwrap();
}
