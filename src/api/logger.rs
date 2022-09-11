use std::convert::Infallible;

use crate::handlers;
use crate::models::sensor_logs::SensorLog;
use serde_derive::{Deserialize, Serialize};
use warp::{hyper::Response, Filter};

#[derive(Debug, Deserialize, Serialize)]
pub struct LogFilterInput {
    pub sensor: Option<u32>,
    pub after: Option<u64>,
    pub before: Option<u64>,
    pub limit: Option<u32>,
    pub station: Option<u32>,
    pub interval: Option<u32>,
}

pub fn log_filters() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    logs().or(logs_csv()).or(log_saves())
}

async fn get_logs_response(input: LogFilterInput) -> Result<impl warp::Reply, Infallible> {
    let sensor_logs = handlers::logger::logs(input);

    Ok(warp::reply::json(&sensor_logs))
}

async fn get_logs_csv_response(input: LogFilterInput) -> Result<impl warp::Reply, Infallible> {
    let log_csv_str = handlers::logger::logs_csv_str(input);

    Ok(Response::builder()
        .status(200)
        .header("Content-Type", "text/csv")
        .header(
            "Content-Disposition",
            format!("attachment; filename=sensor_logs.csv"),
        )
        .body(log_csv_str))
}

// GET /logs
pub fn logs() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("logs")
        .and(warp::get())
        .and(warp::query::<LogFilterInput>())
        .and_then(get_logs_response)
}

pub fn logs_csv() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("logs_csv")
        .and(warp::get())
        .and(warp::query::<LogFilterInput>())
        .and_then(get_logs_csv_response)
}

// POST /logs
pub fn log_saves() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("logs")
        .and(warp::post())
        .and(json_body())
        .and_then(handlers::logger::log_saves)
        .with(warp::log("api"))
}

fn json_body() -> impl Filter<Extract = (Vec<SensorLog>,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}
