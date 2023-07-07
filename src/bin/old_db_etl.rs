extern crate logger;

use crate::logger::db::sqlite::get_db;
use crate::logger::handlers::logger::log_saves;
use crate::logger::models::sensor_logs::SensorLog;
use logger::{models::init_db, utils::iso_date_to_millis};

#[derive(Debug, Clone)]
pub struct OldSGResult {
    id: u64,
    station: u32,
    sensor: u32,
    available: bool,
    valid: bool,
    outdated: bool,
    value: f32,
    created_time: String,
}

fn chunk(vec: &Vec<OldSGResult>, size: usize) -> Vec<Vec<OldSGResult>> {
    vec.chunks(size).map(|chunk| chunk.to_vec()).collect()
}

#[tokio::main]
async fn main() {
    // println!("{}", str_date_to_millis("2020-07-01 00:56:48.714"));

    // return;
    let old_conn = get_db("old-sg-test.db").get().unwrap();
    init_db();

    let stm = String::from("SELECT * FROM sensor_loggers");
    let mut stmt = old_conn.prepare(&stm).unwrap();

    let results = stmt
        .query_map([], |row| {
            Ok(OldSGResult {
                id: row.get(0)?,
                station: row.get(1)?,
                sensor: row.get(2)?,
                available: row.get(3)?,
                valid: row.get(4)?,
                outdated: row.get(5)?,
                value: row.get(6)?,
                created_time: row.get(7)?,
            })
        })
        .unwrap();

    let result_vec = results.collect::<Result<Vec<OldSGResult>, _>>().unwrap();
    let chunked_results = chunk(&result_vec, 1000);

    for result in &chunked_results {
        // map from OldSGResult to SensorLog
        let sensor_logs: Vec<SensorLog> = result
            .iter()
            .map(|result| SensorLog {
                sensor: result.sensor,
                outdated: result.outdated,
                value: result.value,
                created_at: iso_date_to_millis(result.created_time.as_str()).to_string(),
                station: result.station,
            })
            .collect();

        log_saves(sensor_logs).await.unwrap();
    }

    println!("{}", result_vec.len());
}
