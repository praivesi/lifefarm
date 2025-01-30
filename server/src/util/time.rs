use chrono::{Utc, TimeZone, Datelike, NaiveDateTime, Weekday, Duration};

pub fn get_weekday(timestamp: i64) -> Weekday {
    let datetime = Utc.timestamp_opt(timestamp, 0).single().unwrap();

    datetime.weekday()
}

pub fn to_midnight(timestamp: i64) -> i64 {
    let datetime = Utc.timestamp_opt(timestamp, 0).single().unwrap();
    let date = datetime.date_naive();

    Utc.from_utc_datetime(&NaiveDateTime::new(date, chrono::NaiveTime::from_hms_opt(0, 0, 0).unwrap()))
        .timestamp()
}

pub fn to_end_of_day(timestamp: i64) -> i64 {
    let datetime = Utc.timestamp_opt(timestamp, 0).single().unwrap();
    let date = datetime.date_naive();

    Utc.from_utc_datetime(&NaiveDateTime::new(date, chrono::NaiveTime::from_hms_opt(23, 59, 59).unwrap()))
        .timestamp()
}

pub fn add_days_to_utc(timestamp: i64, days: i64) -> i64 {
    let datetime = Utc.timestamp_opt(timestamp, 0).single().unwrap();
    let new_datetime = datetime + Duration::days(days);

    new_datetime.timestamp()
}