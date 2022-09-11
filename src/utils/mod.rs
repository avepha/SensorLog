use chrono::{DateTime, NaiveDateTime, Utc};

pub fn bool_to_string(flag: bool) -> String {
    if flag {
        String::from("true")
    } else {
        String::from("false")
    }
}

pub fn ts_to_iso8601(timestamp_in_secs: i64) -> String {
    let naive = NaiveDateTime::from_timestamp(timestamp_in_secs, 0);
    let datetime: DateTime<Utc> = DateTime::from_utc(naive, Utc);
    datetime.format("%Y-%m-%dT%H:%M:%SZ").to_string()
}

pub fn str_date_to_millis(date: &str) -> u64 {
    let date = NaiveDateTime::parse_from_str(date, "%Y-%m-%d %H:%M:%S%.3f");
    match date {
        Ok(date) => u64::try_from(date.timestamp_millis()).unwrap(),
        Err(e) => {
            println!("{:?}", e);
            0
        }
    }
}
