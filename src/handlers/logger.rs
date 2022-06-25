use crate::db::sqlite::SQLITEPOOL;
use crate::models::sensor_logs::SensorLog;
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

pub async fn log_saves(sensor: SensorLog) -> Result<impl warp::Reply, Infallible> {
    Ok(warp::reply::json(&HashMap::from([(
        "status",
        "not implemented",
    )])))
}
