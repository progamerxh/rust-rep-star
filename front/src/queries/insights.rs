use shared::models::Insight;

use crate::queries::BASE_API_URL;
pub static TESTIMONIAL_API: &str = "insights";

fn insights_endpoint() -> String {
    let window = web_sys::window().expect("no global `window` exists");
    let location = window.location();
    // let host = location.host().expect("should have a host");
    let host = "localhost:8000";
    let protocol = location.protocol().expect("should have a protocol");
    format!(
        "{}//{}/{}/{}",
        protocol, host, BASE_API_URL, TESTIMONIAL_API
    )
}

pub async fn get_insights() -> Result<Vec<Insight>, reqwest::Error> {
    let response = reqwest::get(&insights_endpoint()).await?;
    let insights = response.json::<Vec<Insight>>().await?;
    Ok(insights)
}
