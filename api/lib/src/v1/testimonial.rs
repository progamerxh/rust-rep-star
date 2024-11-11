use actix_web::{
    web::{self, ServiceConfig},
    HttpResponse,
};
use shared::{
    models::{CreateTestimonial, Testimonial},
    queries,
};
use uuid::Uuid;

use crate::testimonial_embedding_repository::TestimonialEmbeddingRepository;
use crate::testimonial_repository::TestimonialRepository;

pub fn service<R: TestimonialRepository, E: TestimonialEmbeddingRepository>(
    cfg: &mut ServiceConfig,
) {
    cfg.service(
        web::scope("/v1/testimonials")
            // get all testimonials
            .route("", web::get().to(get_all::<R, E>))
            // get by id
            .route("/{testimonial_id}", web::get().to(get::<R>))
            // post new testimonial
            .route("", web::post().to(post::<R, E>))
            // update testimonial
            .route("", web::put().to(put::<R>))
            // delete testimonial
            .route("/{testimonial_id}", web::delete().to(delete::<R>)),
    );
}

async fn get_all<R: TestimonialRepository, E: TestimonialEmbeddingRepository>(
    repo: web::Data<R>,
    embedding_repo: web::Data<E>,
    queries: web::Query<queries::TestimonialQueries>,
) -> HttpResponse {
    if queries
        .q
        .as_ref()
        .map(|q| q.trim())
        .filter(|q| !q.is_empty())
        .is_some()
    {
        return match embedding_repo
            .get_testimonial_embeddings(queries.q.clone().unwrap())
            .await
        {
            Ok(embeddings) => {
                let testimonial_ids: Vec<Uuid> = embeddings
                    .iter()
                    .map(|embedding| embedding.testimonial_id)
                    .collect();

                match repo.get_testimonials_by_ids(testimonial_ids).await {
                    Ok(testimonials) => HttpResponse::Ok().json(testimonials),
                    Err(e) => {
                        HttpResponse::NotFound().body(format!("Internal server error: {:?}", e))
                    }
                }
            }
            Err(e) => HttpResponse::NotFound().body(format!("Internal server error: {:?}", e)),
        };
    }

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

async fn post<R: TestimonialRepository, E: TestimonialEmbeddingRepository>(
    testimonial: web::Json<CreateTestimonial>,
    repo: web::Data<R>,
    embedding_repo: web::Data<E>,
) -> HttpResponse {
    let res = match repo.create_testimonial(&testimonial).await {
        Ok(testimonial) => Ok(testimonial),
        Err(e) => Err(e),
    };

    match res {
        Ok(testimonial) => {
            if testimonial.content.len() <= 0 {
                return HttpResponse::Ok().json(testimonial.clone());
            }

            let testimonial_embedding =
                embedding_repo.create_testimonial_embedding(testimonial.clone());

            match testimonial_embedding.await {
                Ok(_) => HttpResponse::Ok().json(testimonial),
                Err(e) => HttpResponse::NotFound().body(format!("Internal server error: {:?}", e)),
            }
        }
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
