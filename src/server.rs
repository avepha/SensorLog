use crate::api;
use crate::models::init_db;
use warp::Filter;

pub fn start() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    init_db();

    let root = warp::path::end().map(|| "SG Logger service");
    let health_check = warp::path("health").map(|| "OK");

    let logs_api = api::logger::log_filters();

    root.or(health_check).or(logs_api.with(warp::log("logs")))
}
