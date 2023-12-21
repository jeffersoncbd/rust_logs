use crate::time;
use chrono::FixedOffset;
use std::{
    fs::{self, File, OpenOptions},
    io::{ErrorKind, Write},
    path::Path,
};

pub fn get_file(time_zone: Option<&FixedOffset>) -> File {
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

    OpenOptions::new()
        .write(true)
        .append(true)
        .open(file_name)
        .unwrap()
}

pub fn writeln(log: &String, time_zone: Option<&FixedOffset>) {
    let mut file = get_file(time_zone);
    let log = log.replace("  ", " ");
    writeln!(file, "{log}").unwrap();
}

pub fn write(log: &String, time_zone: Option<&FixedOffset>) {
    let mut file = get_file(time_zone);
    let log = log.replace("  ", " ");
    write!(file, "{log}").unwrap();
}
