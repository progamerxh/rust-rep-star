use shared::models::{CreateTestimonial, Testimonial};

const API_ENDPOINT: &str = "api/v1";

fn testimonials_endpoint() -> String {
    let window = web_sys::window().expect("no global `window` exists");
    let location = window.location();
    let host = location.host().expect("should have a host");
    let protocol = location.protocol().expect("should have a protocol");
    let endpoint = format!("{}//{}/{}", protocol, host, API_ENDPOINT);
    format!("{}/testimonials", endpoint)
}

pub async fn get_testimonials() -> Result<Vec<Testimonial>, reqwest::Error> {
    log::info!("Fetching testimonials from {}", testimonials_endpoint());
    let response = reqwest::get(&testimonials_endpoint()).await?;
    let testimonials = response.json::<Vec<Testimonial>>().await?;
    Ok(testimonials)
}

pub async fn create_testimonial(
    create_testimonial: CreateTestimonial,
) -> Result<Testimonial, reqwest::Error> {
    let endpoint = testimonials_endpoint();
    log::info!("Creating testimonial at {}", endpoint);

    let client = reqwest::Client::new();
    let response = client
        .post(&endpoint)
        .json(&create_testimonial)
        .send()
        .await?;
    let testimonial = response.json::<Testimonial>().await?;
    Ok(testimonial)
}
