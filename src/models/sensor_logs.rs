use crate::db::sqlite::SQLITEPOOL;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SensorLog {
    pub sensor: u32,
    pub outdated: bool,
    pub value: f32,
    pub created_at: String,
    pub station: u32,
}

pub fn setup() {
    let conn = SQLITEPOOL.get().unwrap();
    conn.execute(
        &"
        CREATE TABLE IF NOT EXISTS sensor_logs (
            sensor INTEGER NOT NULL,
            outdated BOOL NOT NULL,
            station INTEGER NOT NULL,
            value FLOAT CHECK (value >= 0.0) NOT NULL,
            created_at INTEGER NOT NULL,
            PRIMARY KEY (sensor, station, created_at) 
        ) 
    ",
        [],
    )
    .unwrap();
}
