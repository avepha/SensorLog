pub mod api;
pub mod db;
pub mod handlers;
pub mod models;
pub mod utils;

use crate::models::sensor_logs;
use warp::Filter;

#[macro_use]
extern crate lazy_static;

#[tokio::main]
async fn main() {
    sensor_logs::setup();

    let root = warp::path::end().map(|| "SG Logger service");
    let health_check = warp::path("health").map(|| "OK");

    let logs_api = api::logger::log_filters();
    let routes = root.or(health_check).or(logs_api.with(warp::log("logs")));

    println!("Logger service is ready at http://localhost:3030");
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
