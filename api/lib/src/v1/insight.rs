use actix_web::{
    web::{self, ServiceConfig},
    HttpResponse,
};

use crate::insight_repository::InsightRepository;

pub fn service<R: InsightRepository>(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/v1/insights")
            // get all insights
            .route("", web::get().to(get_all::<R>)),
    );
}

async fn get_all<R: InsightRepository>(repo: web::Data<R>) -> HttpResponse {
    match repo.get_insights().await {
        Ok(films) => HttpResponse::Ok().json(films),
        Err(e) => HttpResponse::NotFound().body(format!("Internal server error: {:?}", e)),
    }
}
