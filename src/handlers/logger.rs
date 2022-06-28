use crate::api::logger::LogFilterInput;
use crate::db::sqlite::SQLITEPOOL;
use crate::models::sensor_logs::SensorLog;
use crate::utils::ts_to_iso8601;

use chrono::offset;
use rusqlite::params_from_iter;
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;
use std::convert::Infallible;

#[derive(Debug, Deserialize, Serialize, Clone)]
struct SensorLogResponse {
    sensor: u32,
    station: u32,
    outdated: bool,
    value: f32,
    created_at: String,
    created_at_ts: i64,
}

pub async fn logs(params: LogFilterInput) -> Result<impl warp::Reply, Infallible> {
    let mut base_stm =
        String::from("SELECT sensor, station, outdated, value, created_at FROM sensor_logs");

    let limit = match params.limit {
        Some(limit) => limit,
        None => 10,
    };

    let mut conditions: Vec<String> = Vec::new();

    if params.sensor != None {
        conditions.push(format!("sensor = {}", params.sensor.unwrap()));
    }

    if params.station != None {
        conditions.push(format!("station = '{}'", params.station.unwrap()));
    }

    if params.after != None && params.before == None {
        conditions.push(format!(" created_at > {} ", params.after.unwrap()));
    } else if params.after == None && params.before != None {
        conditions.push(format!(" created_at < {} ", params.before.unwrap()));
    } else if params.after != None && params.before != None {
        conditions.push(format!(
            " created_at BETWEEN {} AND {} ",
            params.after.unwrap(),
            params.before.unwrap()
        ));
    }

    if conditions.len() > 0 {
        base_stm.push_str(" WHERE ");
        base_stm.push_str(&conditions.join(" AND "));
    }

    base_stm.push_str(&format!(" LIMIT {}", limit));

    println!("[Query] {}", base_stm);

    let conn = SQLITEPOOL.get().unwrap();
    let mut stmt = conn.prepare(&base_stm).unwrap();
    let results = stmt
        .query_map([], |row| {
            let timestamp: i64 = match row.get(4) {
                Ok(ts) => ts,
                Err(e) => {
                    println!("[Error] {}", e);
                    0
                }
            };

            Ok(SensorLogResponse {
                sensor: row.get(0)?,
                station: row.get(1)?,
                outdated: row.get(2)?,
                value: row.get(3)?,
                created_at: ts_to_iso8601(timestamp / 1000),
                created_at_ts: timestamp,
            })
        })
        .unwrap();

    let mut sensor_logs: Vec<SensorLogResponse> = Vec::new();
    for r in results {
        sensor_logs.push(r.unwrap())
    }

    Ok(warp::reply::json(&sensor_logs))
}

pub async fn log_saves(sensors: Vec<SensorLog>) -> Result<impl warp::Reply, Infallible> {
    let conn = SQLITEPOOL.get().unwrap();

    println!("[Info] Saving... {:?}", sensors);

    let mut values: Vec<String> = Vec::new();
    let mut placeholers = String::from(
        "INSERT INTO sensor_logs (sensor, station, outdated, value, created_at) VALUES",
    );

    for (pos, sensor) in sensors.iter().enumerate() {
        let start_pos = 5 * pos;
        placeholers = format!(
            "{}{} (?{}, ?{}, ?{}, ?{}, ?{})",
            placeholers,
            if pos == 0 { "" } else { "," },
            start_pos + 1,
            start_pos + 2,
            start_pos + 3,
            start_pos + 4,
            start_pos + 5,
        );

        values.push(sensor.sensor.to_string());
        values.push(sensor.station.to_string());
        values.push(if sensor.outdated {
            String::from("1")
        } else {
            String::from("0")
        });
        values.push(sensor.value.to_string());
        values.push(offset::Utc::now().timestamp_millis().to_string());
    }

    let result = conn.execute(&placeholers, params_from_iter(values.iter()));

    match result {
        Ok(usize) => Ok(warp::reply::json(&HashMap::from([(
            "effected_rows",
            usize,
        )]))),
        Err(err) => Ok(warp::reply::json(&HashMap::from([
            ("error", err.to_string()),
            ("sql", placeholers),
            ("values", values.concat().to_string()),
        ]))),
    }
}
