use crate::db::sqlite::SQLITEPOOL;
use crate::models::sensor_logs::SensorLog;
use rusqlite::params_from_iter;
use std::collections::HashMap;
use std::convert::Infallible;

pub async fn logs() -> Result<impl warp::Reply, Infallible> {
    let conn = SQLITEPOOL.get().unwrap();
    let mut stmt = conn.prepare(&"SELECT * FROM sensor_logs;").unwrap();
    let results = stmt
        .query_map([], |row| {
            Ok(SensorLog {
                sensor: row.get(0)?,
                outdated: row.get(1)?,
                value: row.get(2)?,
                created_at: row.get(3)?,
            })
        })
        .unwrap();

    let mut sensor_logs: Vec<SensorLog> = Vec::new();
    for r in results {
        sensor_logs.push(r.unwrap())
    }

    Ok(warp::reply::json(&sensor_logs))
}

pub async fn log_saves(sensors: Vec<SensorLog>) -> Result<impl warp::Reply, Infallible> {
    let conn = SQLITEPOOL.get().unwrap();

    let mut values: Vec<String> = Vec::new();
    let mut placeholers =
        String::from("INSERT INTO sensor_logs (sensor, outdated, value, created_at) VALUES");

    for (pos, sensor) in sensors.iter().enumerate() {
        let start_pos = 3 * pos;

        placeholers = format!(
            "{}{} (?{}, ?{}, ?{}, CURRENT_TIMESTAMP)",
            placeholers,
            if pos == 0 { "" } else { "," },
            start_pos + 1,
            start_pos + 2,
            start_pos + 3
        );

        values.push(sensor.sensor.to_string());
        values.push(if sensor.outdated {
            String::from("1")
        } else {
            String::from("0")
        });
        values.push(sensor.value.to_string());
    }

    let result = conn
        .execute(&placeholers, params_from_iter(values.iter()))
        .unwrap();

    Ok(warp::reply::json(&HashMap::from([(
        "effected_rows",
        result,
    )])))
}
