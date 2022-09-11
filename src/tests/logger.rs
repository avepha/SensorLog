use crate::api::logger;
use crate::models::sensor_logs::SensorLog;
use crate::tests::utils::{clean_up, initialize};
use warp::test::request;

#[tokio::test]
async fn save_logs_without_filters() {
    initialize();
    clean_up();
    let save_logs = logger::log_saves();
    let body: Vec<SensorLog> = vec![
        SensorLog {
            sensor: 2,
            station: 2,
            outdated: false,
            value: 2.22,
            created_at: 2222222222,
        },
        SensorLog {
            sensor: 1,
            station: 1,
            outdated: false,
            value: 1.11,
            created_at: 1111111111,
        },
    ];

    let response = request()
        .method("POST")
        .path("/logs")
        .json(&body)
        .reply(&save_logs)
        .await;

    assert_eq!(response.status(), 200);
    assert_eq!(response.body(), "{\"effected_rows\":2}");

    let get_logs = logger::logs();
    let resp = request().method("GET").path("/logs").reply(&get_logs).await;

    assert_eq!(resp.status(), 200);
}
