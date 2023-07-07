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
        CREATE TABLE IF NOT EXISTS logs (
            created_at INTEGER NOT NULL,
            sensor INTEGER NOT NULL,
            value FLOAT NOT NULL,
            outdated BOOL NOT NULL,
            station INTEGER CHECK (value >= 0) NOT NULL,
            PRIMARY KEY (sensor, station, created_at)
        )
    ",
        [],
    )
    .unwrap();

    conn.execute(
        &"CREATE INDEX IF NOT EXISTS logs_created_at ON logs (created_at)",
        [],
    )
    .unwrap();
}
