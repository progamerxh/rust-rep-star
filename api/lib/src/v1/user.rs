use actix_web::{
    web::{self, ServiceConfig},
    HttpResponse,
};
use shared::models::{CreateUser, User};
use uuid::Uuid;

use crate::user_repository::UserRepository;

pub fn service<R: UserRepository>(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/v1/users")
            // get all users
            .route("", web::get().to(get_all::<R>))
            // get by id
            .route("/{user_id}", web::get().to(get::<R>))
            // post new user
            .route("", web::post().to(post::<R>))
            // update user
            .route("", web::put().to(put::<R>))
            // delete user
            .route("/{user_id}", web::delete().to(delete::<R>)),
    );
}

async fn get_all<R: UserRepository>(repo: web::Data<R>) -> HttpResponse {
    match repo.get_users().await {
        Ok(films) => HttpResponse::Ok().json(films),
        Err(e) => HttpResponse::NotFound().body(format!("Internal server error: {:?}", e)),
    }
}

async fn get<R: UserRepository>(user_id: web::Path<Uuid>, repo: web::Data<R>) -> HttpResponse {
    match repo.get_user(&user_id).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => HttpResponse::NotFound().body(format!("Internal server error: {:?}", e)),
    }
}

async fn post<R: UserRepository>(user: web::Json<CreateUser>, repo: web::Data<R>) -> HttpResponse {
    match repo.create_user(&user).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => HttpResponse::NotFound().body(format!("Internal server error: {:?}", e)),
    }
}

async fn put<R: UserRepository>(user: web::Json<User>, repo: web::Data<R>) -> HttpResponse {
    match repo.update_user(&user).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => HttpResponse::NotFound().body(format!("Internal server error: {:?}", e)),
    }
}

async fn delete<R: UserRepository>(user_id: web::Path<Uuid>, repo: web::Data<R>) -> HttpResponse {
    match repo.delete_user(&user_id).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => HttpResponse::NotFound().body(format!("Internal server error: {:?}", e)),
    }
}
