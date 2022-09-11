use crate::{db::sqlite::SQLITEPOOL, models::init_db};
use std::sync::Once;

static INIT: Once = Once::new();

pub fn initialize() {
    INIT.call_once(|| {
        init_db();
    });
}

pub fn clean_up() {
    SQLITEPOOL
        .get()
        .unwrap()
        .execute("DELETE FROM logs", [])
        .unwrap();
}
