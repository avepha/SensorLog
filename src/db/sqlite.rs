use r2d2;
use r2d2_sqlite::SqliteConnectionManager;

pub type SqlitePool = r2d2::Pool<SqliteConnectionManager>;

lazy_static! {
    pub static ref SQLITEPOOL: SqlitePool = {
        let sqlite_database = "sensor_logs.db";
        let manager = SqliteConnectionManager::file(&sqlite_database);
        let pool = r2d2::Pool::builder().build(manager).unwrap();
        pool
    };
}
