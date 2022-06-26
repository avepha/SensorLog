use crate::handlers;
use crate::models::sensor_logs::SensorLog;
use serde_derive::{Deserialize, Serialize};
use warp::Filter;

#[derive(Debug, Deserialize, Serialize)]
pub struct LogFilterInput {
    pub sensor: Option<u32>,
    pub after: Option<u64>,
    pub before: Option<u64>,
    pub limit: Option<u32>,
}

pub fn log_filters() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    logs().or(log_saves())
}

// GET /logs
pub fn logs() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("logs")
        .and(warp::get())
        .and(warp::query::<LogFilterInput>())
        .and_then(handlers::logger::logs)
}

// POST /logs
pub fn log_saves() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("logs")
        .and(warp::post())
        .and(json_body())
        .and_then(handlers::logger::log_saves)
}

fn json_body() -> impl Filter<Extract = (Vec<SensorLog>,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}
