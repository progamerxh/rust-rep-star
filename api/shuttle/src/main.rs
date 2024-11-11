use actix_files::Files;
use actix_web::web::{self, ServiceConfig};
use shuttle_actix_web::ShuttleActixWeb;
use shuttle_runtime::CustomError;
use sqlx::migrate::Migrator;
use sqlx::postgres::PgPoolOptions;
use std::time::Duration;

pub static MIGRATIONS: Migrator = sqlx::migrate!("../migrations");

#[shuttle_runtime::main]
async fn server() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    // loads env variables from .env
    dotenv::dotenv().ok();
    let database_url =
        std::env::var("DATABASE_URL").expect("DATABASE_URL must be set in the environment");

    let pool = PgPoolOptions::new()
        .max_connections(5) // ollama local is quite slow
        .acquire_slow_threshold(Duration::from_secs(10))
        .connect(&database_url)
        .await
        .map_err(CustomError::new)?;

    let testimonial_repository =
        api_lib::testimonial_repository::PostgresTestimonialRepository::new(pool.clone());
    let testimonial_repository = actix_web::web::Data::new(testimonial_repository);

    let testimonial_embedding_repository =
        api_lib::testimonial_embedding_repository::PostgresTestimonialEmbeddingRepository::new(
            pool.clone(),
        );
    let testimonial_embedding_repository =
        actix_web::web::Data::new(testimonial_embedding_repository);

    let metric_repository = api_lib::metric_repository::PostgresMetricRepository::new(pool.clone());
    let metric_repository = actix_web::web::Data::new(metric_repository);

    let user_repository = api_lib::user_repository::PostgresUserRepository::new(pool.clone());
    let user_repository = actix_web::web::Data::new(user_repository);

    let insight_repository =
        api_lib::insight_repository::PostgresInsightRepository::new(pool.clone());
    let insight_repository = actix_web::web::Data::new(insight_repository);

    tracing::info!("Database connection pool created successfully!");

    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(
            web::scope("/api")
                .app_data(testimonial_repository)
                .app_data(testimonial_embedding_repository)
                .app_data(metric_repository)
                .app_data(user_repository)
                .app_data(insight_repository)
                .configure(api_lib::health::service)
                .configure(
                    api_lib::v1::testimonial::service::<
                        api_lib::testimonial_repository::PostgresTestimonialRepository,
                        api_lib::testimonial_embedding_repository::PostgresTestimonialEmbeddingRepository,
                    >,
                )
                .configure(
                    api_lib::v1::user::service::<api_lib::user_repository::PostgresUserRepository>,
                )
                .configure(
                    api_lib::v1::metric::service::<
                        api_lib::metric_repository::PostgresMetricRepository,
                    >,
                )
                .configure(
                    api_lib::v1::insight::service::<
                        api_lib::insight_repository::PostgresInsightRepository,
                        api_lib::testimonial_repository::PostgresTestimonialRepository,
                    >,
                ),
        )
        .service(
            Files::new("/", "static")
                .show_files_listing()
                .index_file("index.html"),
        );
    };

    Ok(config.into())
}
