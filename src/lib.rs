#![allow(clippy::ptr_arg)]
mod file;
mod time;

use std::{fmt, panic::Location, process};

pub use chrono::FixedOffset;

fn mount_log(tag: &str, message: impl fmt::Display, time_zone: Option<&FixedOffset>) -> String {
    format!(
        "🔹 [{}] {} - {}",
        time::now(time_zone).get_date_and_hour(),
        tag,
        message
    )
}

pub struct Logger {
    write_in_files: bool,
    time_zone: Option<FixedOffset>,
}
impl Logger {
    pub fn new(write_in_files: bool, time_zone: Option<FixedOffset>) -> Logger {
        Logger {
            write_in_files,
            time_zone,
        }
    }

    fn get_time_zone(&self) -> Option<&FixedOffset> {
        match &self.time_zone {
            Some(time_zone) => Some(time_zone),
            None => None,
        }
    }

    pub fn clear_terminal(&self) {
        print!("\x1B[2J\x1B[1;1H");
    }

    pub fn log(&self, tag: &str, message: impl fmt::Display) {
        let time_zone = self.get_time_zone();
        let log = mount_log(tag, message, time_zone);
        if self.write_in_files {
            file::write(&log, time_zone)
        }
        println!("{log}");
    }

    pub fn println(&self, content: impl fmt::Display) {
        let content = content.to_string();
        let time_zone = self.get_time_zone();
        println!("{}", content);
        if self.write_in_files {
            file::write(&content, time_zone)
        }
    }

    #[track_caller]
    pub fn throw_error(&self, error: impl fmt::Display, description: impl fmt::Display) {
        let location = Location::caller();
        let time_zone = self.get_time_zone();
        let log = format!(
            "\n❌ [{}] CRITICAL ERROR - {}: {}\n❌ {}\n",
            time::now(time_zone).get_date_and_hour(),
            location,
            error,
            description
        );
        eprintln!("\x1b[0;31m{}\x1b[0m", &log);
        if self.write_in_files {
            file::write(&log, time_zone)
        }
        process::exit(1);
    }
}
