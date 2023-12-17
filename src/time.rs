use chrono::{
    format::{DelayedFormat, StrftimeItems},
    DateTime, FixedOffset, Utc,
};

pub enum NowDateTime {
    FixedOffset(DateTime<FixedOffset>),
    Utc(DateTime<Utc>),
}

pub struct Now {
    date_time: NowDateTime,
}
impl Now {
    pub fn new(date_time: NowDateTime) -> Now {
        Now { date_time }
    }
    pub fn get_date(&self) -> DelayedFormat<StrftimeItems> {
        match self.date_time {
            NowDateTime::FixedOffset(now) => now.format("%Y-%m-%d"),
            NowDateTime::Utc(now) => now.format("%Y-%m-%d"),
        }
    }
    pub fn get_date_and_hour(&self) -> DelayedFormat<StrftimeItems> {
        match self.date_time {
            NowDateTime::FixedOffset(now) => now.format("%Y-%m-%d %H:%M:%S"),
            NowDateTime::Utc(now) => now.format("%Y-%m-%d %H:%M:%S"),
        }
    }
}

pub fn now(time_zone: Option<&FixedOffset>) -> Now {
    let now = Utc::now();
    let now = match time_zone {
        Some(time_zone) => NowDateTime::FixedOffset(now.with_timezone(time_zone)),
        None => NowDateTime::Utc(now),
    };

    Now::new(now)
}
