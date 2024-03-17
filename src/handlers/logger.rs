use std::collections::HashMap;

use rusqlite::params_from_iter;
use serde_derive::{Deserialize, Serialize};
use warp::Rejection;

use crate::api::logger::LogFilterInput;
use crate::db::sqlite::SQLITEPOOL;
use crate::models::sensor_logs::SensorLog;
use crate::utils::{iso_date_to_millis, ts_to_iso8601};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SensorLogResponse {
    pub sensor: u32,
    pub station: u32,
    pub outdated: bool,
    pub value: f32,
    pub created_at: String,
    pub created_at_ts: i64,
}

pub fn logs(params: LogFilterInput) -> Vec<SensorLogResponse> {
    let mut base_stm =
        String::from("SELECT sensor, station, outdated, value, created_at FROM logs");

    let limit = match params.limit {
        Some(limit) => limit,
        None => 10,
    };

    let interval = match params.interval {
        Some(interval) => interval,
        None => 1,
    };

    let mut conditions: Vec<String> = Vec::new();

    if params.sensor != None {
        conditions.push(format!("sensor = {}", params.sensor.unwrap()));
    }

    if params.station != None {
        conditions.push(format!("station = '{}'", params.station.unwrap()));
    }

    if params.after != None && params.before == None {
        conditions.push(format!(" created_at > {} ", iso_date_to_millis(params.after.unwrap().as_str())));
    } else if params.after == None && params.before != None {
        conditions.push(format!(" created_at < {} ", iso_date_to_millis(params.before.unwrap().as_str())));
    } else if params.after != None && params.before != None {
        conditions.push(format!(
            " created_at BETWEEN {} AND {} ",
            iso_date_to_millis(params.after.unwrap().as_str()),
            iso_date_to_millis(params.before.unwrap().as_str())
        ));
    }

    if conditions.len() > 0 {
        base_stm.push_str(" WHERE ");
        base_stm.push_str(&conditions.join(" AND "));
    }

    base_stm.push_str(&format!(" LIMIT {}", 99999));

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

    let mut count = 0;
    let mut limit_counter = 0;
    for r in results {
        count += 1;

        if count == interval {
            sensor_logs.push(r.unwrap());
            limit_counter += 1;
            count = 0;

            if limit_counter == limit {
                break;
            }
        }
    }

    sensor_logs
}

pub async fn log_saves(sensors: Vec<SensorLog>) -> Result<impl warp::Reply, Rejection> {
    let conn = SQLITEPOOL.get().unwrap();

    println!("[Info] Saving... {:?}", sensors);

    let mut values: Vec<String> = Vec::new();
    let mut placeholers = String::from(
        "INSERT OR IGNORE INTO logs (sensor, station, outdated, value, created_at) VALUES",
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

        let timestamp = iso_date_to_millis(&sensor.created_at);

        if timestamp == -1 {
            return Ok(warp::reply::json(&HashMap::from([
                ("error", format!("Invalid date format {}", sensor.created_at)),
            ])))
        }

        values.push(timestamp.to_string());
    }

    let result = conn.execute(&placeholers, params_from_iter(values.iter()));

    return match result {
        Ok(usize) => {
            Ok(warp::reply::json(&HashMap::from([
                ("effected_rows", usize)
            ])))
        }
        Err(err) => {
            println!("[Error] {}", err);

            Ok(warp::reply::json(&HashMap::from([
                ("error", err.to_string()),
                ("sql", placeholers),
                ("values", values.concat().to_string()),
            ])))
        }
    }
}

pub fn logs_csv_str<'a>(params: LogFilterInput) -> String {
    let sensor_logs = logs(params);
    let mut wtr = csv::Writer::from_writer(vec![]);

    match wtr.write_record(&["Sensor name", "Station", "validity", "value", "logged time"]) {
        Err(e) => {
            println!("{}", e);
        }
        Ok(_) => {}
    }

    for sensor_log in sensor_logs {
        match wtr.serialize((
            sensor_log.sensor,
            sensor_log.station,
            sensor_log.outdated,
            sensor_log.value,
            sensor_log.created_at,
        )) {
            Err(e) => eprintln!("{}", e),
            Ok(_) => {}
        }
    }

    let result = String::from_utf8(wtr.into_inner().unwrap());

    match result {
        Err(e) => {
            eprintln!("{}", e);

            String::from("")
        }
        Ok(str) => str,
    }
}
