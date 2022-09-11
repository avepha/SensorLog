use lazy_static::lazy_static;
use r2d2;
use r2d2_sqlite::SqliteConnectionManager;
use std::env;

pub type SqlitePool = r2d2::Pool<SqliteConnectionManager>;

pub fn get_db(db_name: &str) -> r2d2::Pool<SqliteConnectionManager> {
    let manager = SqliteConnectionManager::file(db_name);
    let pool = r2d2::Pool::builder().build(manager).unwrap();

    pool
}

lazy_static! {
    pub static ref SQLITEPOOL: SqlitePool = {
        let db_name = match env::var("DB_NAME") {
            Ok(val) => val,
            Err(_) => "sensor_log.db".to_string(),
        };

        println!("[Info] Using database: {}", db_name);

        let manager = SqliteConnectionManager::file(&db_name.as_str());
        let pool = r2d2::Pool::builder().build(manager).unwrap();
        pool
    };
}
