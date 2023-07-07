use chrono::{DateTime, NaiveDateTime, Utc};

pub fn bool_to_string(flag: bool) -> String {
    if flag {
        String::from("true")
    } else {
        String::from("false")
    }
}

pub fn is_valid_iso_date_format(date: &str) -> bool {
    let is_iso_string_format = match DateTime::parse_from_rfc3339(date) {
        Ok(_) => true,
        Err(_) => false,
    };

    is_iso_string_format
}

pub fn ts_to_iso8601(timestamp_in_secs: i64) -> String {
    let naive = NaiveDateTime::from_timestamp(timestamp_in_secs, 0);
    let datetime: DateTime<Utc> = DateTime::from_utc(naive, Utc);
    datetime.format("%Y-%m-%dT%H:%M:%SZ").to_string()
}

pub fn iso_date_to_millis(date: &str) -> i64 {
    match DateTime::parse_from_rfc3339(date) {
        Ok(value) => value.timestamp_millis(),
        Err(_) => -1,
    }
}
