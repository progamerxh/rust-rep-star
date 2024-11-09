use actix_web::{
    web::{self, ServiceConfig},
    HttpResponse,
};
use shared::models::{CreateTestimonial, Testimonial};
use uuid::Uuid;

use crate::testimonial_repository::TestimonialRepository;

pub fn service<R: TestimonialRepository>(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/v1/testimonials")
            // get all testimonials
            .route("", web::get().to(get_all::<R>))
            // get by id
            .route("/{testimonial_id}", web::get().to(get::<R>))
            // post new testimonial
            .route("", web::post().to(post::<R>))
            // update testimonial
            .route("", web::put().to(put::<R>))
            // delete testimonial
            .route("/{testimonial_id}", web::delete().to(delete::<R>)),
    );
}

async fn get_all<R: TestimonialRepository>(repo: web::Data<R>) -> HttpResponse {
    match repo.get_testimonials().await {
        Ok(films) => HttpResponse::Ok().json(films),
        Err(e) => HttpResponse::NotFound().body(format!("Internal server error: {:?}", e)),
    }
}

async fn get<R: TestimonialRepository>(
    testimonial_id: web::Path<Uuid>,
    repo: web::Data<R>,
) -> HttpResponse {
    match repo.get_testimonial(&testimonial_id).await {
        Ok(testimonial) => HttpResponse::Ok().json(testimonial),
        Err(e) => HttpResponse::NotFound().body(format!("Internal server error: {:?}", e)),
    }
}

async fn post<R: TestimonialRepository>(
    testimonial: web::Json<CreateTestimonial>,
    repo: web::Data<R>,
) -> HttpResponse {
    match repo.create_testimonial(&testimonial).await {
        Ok(testimonial) => HttpResponse::Ok().json(testimonial),
        Err(e) => HttpResponse::NotFound().body(format!("Internal server error: {:?}", e)),
    }
}

async fn put<R: TestimonialRepository>(
    testimonial: web::Json<Testimonial>,
    repo: web::Data<R>,
) -> HttpResponse {
    match repo.update_testimonial(&testimonial).await {
        Ok(testimonial) => HttpResponse::Ok().json(testimonial),
        Err(e) => HttpResponse::NotFound().body(format!("Internal server error: {:?}", e)),
    }
}

async fn delete<R: TestimonialRepository>(
    testimonial_id: web::Path<Uuid>,
    repo: web::Data<R>,
) -> HttpResponse {
    match repo.delete_testimonial(&testimonial_id).await {
        Ok(testimonial) => HttpResponse::Ok().json(testimonial),
        Err(e) => HttpResponse::NotFound().body(format!("Internal server error: {:?}", e)),
    }
}
