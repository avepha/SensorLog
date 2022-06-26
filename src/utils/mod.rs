use chrono::prelude::{DateTime, Utc};

pub fn bool_to_string(flag: bool) -> String {
    if flag {
        String::from("true")
    } else {
        String::from("false")
    }
}

pub fn iso8601(st: &std::time::SystemTime) -> String {
    let dt: DateTime<Utc> = st.clone().into();
    format!("{}", dt.to_rfc3339())
}
