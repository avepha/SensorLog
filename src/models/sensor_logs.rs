use crate::db::sqlite::SQLITEPOOL;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SensorLog {
    pub sensor: u32,
    pub outdated: bool,
    pub value: f32,
    pub created_at: String,
}

pub fn setup() {
    let conn = SQLITEPOOL.get().unwrap();
    conn.execute(
        &"
        CREATE TABLE IF NOT EXISTS sensor_logs (
            sensor INTEGER NOT NULL,
            outdated BOOL NOT NULL,
            value FLOAT CHECK (value >= 0.0) NOT NULL,
            created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
            PRIMARY KEY (sensor, created_at) 
        ) 
    ",
        [],
    )
    .unwrap();
}
