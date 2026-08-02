use chrono::{Utc, TimeZone, Datelike, NaiveDateTime, Weekday, Duration, FixedOffset};

// Fixed UTC+9 (Korea Standard Time, no DST) — used for all "which calendar day"
// boundary math so daily check-ins line up with the user's local day.
fn kst() -> FixedOffset {
    FixedOffset::east_opt(9 * 3600).unwrap()
}

pub fn get_weekday(timestamp: i64) -> Weekday {
    let datetime = Utc.timestamp_opt(timestamp, 0).single().unwrap().with_timezone(&kst());

    datetime.weekday()
}

pub fn to_midnight(timestamp: i64) -> i64 {
    let datetime = Utc.timestamp_opt(timestamp, 0).single().unwrap().with_timezone(&kst());
    let date = datetime.date_naive();

    kst().from_local_datetime(&NaiveDateTime::new(date, chrono::NaiveTime::from_hms_opt(0, 0, 0).unwrap()))
        .unwrap()
        .timestamp()
}

pub fn to_end_of_day(timestamp: i64) -> i64 {
    let datetime = Utc.timestamp_opt(timestamp, 0).single().unwrap().with_timezone(&kst());
    let date = datetime.date_naive();

    kst().from_local_datetime(&NaiveDateTime::new(date, chrono::NaiveTime::from_hms_opt(23, 59, 59).unwrap()))
        .unwrap()
        .timestamp()
}

pub fn add_days_to_utc(timestamp: i64, days: i64) -> i64 {
    let datetime = Utc.timestamp_opt(timestamp, 0).single().unwrap();
    let new_datetime = datetime + Duration::days(days);

    new_datetime.timestamp()
}

pub fn today_midnight() -> i64 {
    to_midnight(Utc::now().timestamp())
}
