use actix_web::{
    web::{self, ServiceConfig},
    HttpResponse,
};
use shared::models::{CreateTestimonial, Testimonial};
use uuid::Uuid;

use crate::testimonial_repository::TestimonialRepository;

type Repository = web::Data<Box<dyn TestimonialRepository>>;

pub fn service(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/v1/testimonials")
            // get all testimonials
            .route("", web::get().to(get_all))
            // get by id
            .route("/{testimonial_id}", web::get().to(get))
            // post new testimonial
            .route("", web::post().to(post))
            // update testimonial
            .route("", web::put().to(put))
            // delete testimonial
            .route("/{testimonial_id}", web::delete().to(delete)),
    );
}

async fn get_all(repo: Repository) -> HttpResponse {
    match repo.get_testimonials().await {
        Ok(films) => HttpResponse::Ok().json(films),
        Err(e) => HttpResponse::NotFound().body(format!("Internal server error: {:?}", e)),
    }
}

async fn get(testimonial_id: web::Path<Uuid>, repo: Repository) -> HttpResponse {
    match repo.get_testimonial(&testimonial_id).await {
        Ok(testimonial) => HttpResponse::Ok().json(testimonial),
        Err(e) => HttpResponse::NotFound().body(format!("Internal server error: {:?}", e)),
    }
}

async fn post(testimonial: web::Json<CreateTestimonial>, repo: Repository) -> HttpResponse {
    match repo.create_testimonial(&testimonial).await {
        Ok(testimonial) => HttpResponse::Ok().json(testimonial),
        Err(e) => HttpResponse::NotFound().body(format!("Internal server error: {:?}", e)),
    }
}

async fn put(testimonial: web::Json<Testimonial>, repo: Repository) -> HttpResponse {
    match repo.update_testimonial(&testimonial).await {
        Ok(testimonial) => HttpResponse::Ok().json(testimonial),
        Err(e) => HttpResponse::NotFound().body(format!("Internal server error: {:?}", e)),
    }
}

async fn delete(testimonial_id: web::Path<Uuid>, repo: Repository) -> HttpResponse {
    match repo.delete_testimonial(&testimonial_id).await {
        Ok(testimonial) => HttpResponse::Ok().json(testimonial),
        Err(e) => HttpResponse::NotFound().body(format!("Internal server error: {:?}", e)),
    }
}
