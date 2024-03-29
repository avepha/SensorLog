use std::convert::Infallible;

use serde_derive::{Deserialize, Serialize};
use warp::{Filter, hyper::Response, Rejection, reply::json};
use warp::http::StatusCode;
use warp::reply::with_status;

use crate::handlers;
use crate::models::sensor_logs::SensorLog;
use crate::utils::is_valid_iso_date_format;

#[derive(Debug, Deserialize, Serialize)]
pub struct LogFilterInput {
    pub sensor: Option<u32>,
    pub after: Option<String>,
    pub before: Option<String>,
    pub limit: Option<u32>,
    pub station: Option<u32>,
    pub interval: Option<u32>,
}

pub fn log_filters() -> impl Filter<Extract=impl warp::Reply, Error=Rejection> + Clone {
    logs().or(logs_csv()).or(log_saves())
}

async fn get_logs_response(input: LogFilterInput) -> Result<impl warp::Reply, Rejection> {
    if input.after != None && !is_valid_iso_date_format(input.after.as_ref().unwrap())  {
        let json_response = json(&"Invalid date format. Use ISO 8601 format (YYYY-MM-DDTHH:MM:SSZ) at after field");
        return Ok(with_status(json_response, StatusCode::INTERNAL_SERVER_ERROR));
    }

    if input.before != None && !is_valid_iso_date_format(input.before.as_ref().unwrap()) {
        let json_response = json(&"Invalid date format. Use ISO 8601 format (YYYY-MM-DDTHH:MM:SSZ) at before field");
        return Ok(with_status(json_response, StatusCode::INTERNAL_SERVER_ERROR));
    }

    let sensor_logs = handlers::logger::logs(input);

    Ok(with_status(json(&sensor_logs), StatusCode::OK))
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
