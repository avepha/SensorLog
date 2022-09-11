pub mod sensor_logs;

pub fn init_db() {
    sensor_logs::setup();
    println!("[Info] Database initialized");
}
