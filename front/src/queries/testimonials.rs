use shared::models::{CreateTestimonial, Testimonial};

use crate::queries::BASE_API_URL;
pub static TESTIMONIAL_API: &str = "testimonials";

fn testimonials_endpoint() -> String {
    let window = web_sys::window().expect("no global `window` exists");
    let location = window.location();
    let host = location.host().expect("should have a host");
    let protocol = location.protocol().expect("should have a protocol");
    format!(
        "{}//{}/{}/{}",
        protocol, host, BASE_API_URL, TESTIMONIAL_API
    )
}

pub async fn get_testimonials() -> Result<Vec<Testimonial>, reqwest::Error> {
    let response = reqwest::get(&testimonials_endpoint()).await?;
    let testimonials = response.json::<Vec<Testimonial>>().await?;
    Ok(testimonials)
}

pub async fn create_testimonial(
    create_testimonial: CreateTestimonial,
) -> Result<Testimonial, reqwest::Error> {
    let response = reqwest::Client::new()
        .post(&testimonials_endpoint())
        .json(&create_testimonial)
        .send()
        .await?;
    let testimonial = response.json::<Testimonial>().await?;
    Ok(testimonial)
}