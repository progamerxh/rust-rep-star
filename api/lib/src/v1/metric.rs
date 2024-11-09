use actix_web::{
    web::{self, ServiceConfig},
    HttpResponse,
};
use shared::models::CreateMetric;
use uuid::Uuid;

use crate::metric_repository::MetricRepository;

pub fn service<R: MetricRepository>(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/v1/metrics")
            // get all metrics
            .route("", web::get().to(get_all::<R>))
            // get by id
            .route("/{metric_id}", web::get().to(get::<R>))
            // post new metric
            .route("", web::post().to(post::<R>))
            // delete metric
            .route("/{metric_id}", web::delete().to(delete::<R>)),
    );
}

async fn get_all<R: MetricRepository>(repo: web::Data<R>) -> HttpResponse {
    match repo.get_metrics().await {
        Ok(films) => HttpResponse::Ok().json(films),
        Err(e) => HttpResponse::NotFound().body(format!("Internal server error: {:?}", e)),
    }
}

async fn get<R: MetricRepository>(metric_id: web::Path<Uuid>, repo: web::Data<R>) -> HttpResponse {
    match repo.get_metric(&metric_id).await {
        Ok(metric) => HttpResponse::Ok().json(metric),
        Err(e) => HttpResponse::NotFound().body(format!("Internal server error: {:?}", e)),
    }
}

async fn post<R: MetricRepository>(
    metric: web::Json<CreateMetric>,
    repo: web::Data<R>,
) -> HttpResponse {
    match repo.create_metric(&metric).await {
        Ok(metric) => HttpResponse::Ok().json(metric),
        Err(e) => HttpResponse::NotFound().body(format!("Internal server error: {:?}", e)),
    }
}

async fn delete<R: MetricRepository>(
    metric_id: web::Path<Uuid>,
    repo: web::Data<R>,
) -> HttpResponse {
    match repo.delete_metric(&metric_id).await {
        Ok(metric) => HttpResponse::Ok().json(metric),
        Err(e) => HttpResponse::NotFound().body(format!("Internal server error: {:?}", e)),
    }
}
