use chrono::{NaiveDateTime, Utc};

pub fn get_current_utc_timestamp() -> i64 {
    let now = chrono::NaiveDateTime::from_timestamp(Utc::now().timestamp(), 0);
    now.timestamp()
}

pub fn get_current_utc_date_time(offset_seconds: i64) -> NaiveDateTime {
    chrono::NaiveDateTime::from_timestamp(Utc::now().timestamp() + offset_seconds, 0)
}
