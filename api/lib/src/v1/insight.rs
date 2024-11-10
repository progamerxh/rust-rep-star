use std::str::FromStr;

use crate::testimonial_repository::TestimonialRepository;
use crate::{insight_repository::InsightRepository, testimonial_repository::TimeDuration};
use actix_web::{
    web::{self, ServiceConfig},
    HttpResponse,
};
use serde::Deserialize;

#[derive(Deserialize)]
struct InsightQuery {
    duration: Option<String>,
}

pub fn service<R: InsightRepository, T: TestimonialRepository>(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/v1/insights")
            // get all insights
            .route("", web::get().to(get_all::<R, T>)),
    );
}

async fn get_all<R: InsightRepository, T: TestimonialRepository>(
    repo: web::Data<R>,
    testimonial_repo: web::Data<T>,
    query: web::Query<InsightQuery>,
) -> HttpResponse {
    let duration = TimeDuration::from_str(&query.duration.clone().unwrap_or("day".to_string()));

    let testimonials = match testimonial_repo
        .get_testimonials_by_time_duration(duration.unwrap_or(TimeDuration::LastDay))
        .await
    {
        Ok(t) => t,
        Err(e) => {
            return HttpResponse::InternalServerError()
                .body(format!("Error fetching testimonials: {:?}", e))
        }
    };

    match repo.get_insights(testimonials).await {
        Ok(films) => HttpResponse::Ok().json(films),
        Err(e) => HttpResponse::NotFound().body(format!("Internal server error: {:?}", e)),
    }
}
