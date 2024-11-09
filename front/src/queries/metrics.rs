use shared::models::{CreateMetric, Metric};

use crate::queries::BASE_API_URL;
pub static METRIC_API: &str = "metrics";

fn metrics_endpoint() -> String {
    let window = web_sys::window().expect("no global `window` exists");
    let location = window.location();
    // let host = location.host().expect("should have a host");
    let host = "localhost:8000";
    let protocol = location.protocol().expect("should have a protocol");
    format!("{}//{}/{}/{}", protocol, host, BASE_API_URL, METRIC_API)
}

pub async fn get_metrics() -> Result<Vec<Metric>, reqwest::Error> {
    let response = reqwest::get(&metrics_endpoint()).await?;
    let metrics = response.json::<Vec<Metric>>().await?;
    Ok(metrics)
}

pub async fn create_metric(create_metric: CreateMetric) -> Result<Metric, reqwest::Error> {
    let response = reqwest::Client::new()
        .post(&metrics_endpoint())
        .json(&create_metric)
        .send()
        .await?;
    let metric = response.json::<Metric>().await?;
    Ok(metric)
}
