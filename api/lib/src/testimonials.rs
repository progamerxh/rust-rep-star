use actix_web::{
    web::{self, ServiceConfig},
    HttpResponse,
};

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

async fn get_all() -> HttpResponse {
    HttpResponse::Ok().finish()
}

async fn get() -> HttpResponse {
    HttpResponse::Ok().finish()
}

async fn post() -> HttpResponse {
    HttpResponse::Ok().finish()
}

async fn put() -> HttpResponse {
    HttpResponse::Ok().finish()
}

async fn delete() -> HttpResponse {
    HttpResponse::Ok().finish()
}
