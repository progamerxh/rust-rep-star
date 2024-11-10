use std::time::Duration;

use actix_cors::Cors;
use actix_files::Files;
use actix_web::{self, web, App, HttpServer};
use sqlx::migrate::Migrator;
use sqlx::postgres::PgPoolOptions;

pub static MIGRATIONS: Migrator = sqlx::migrate!("../migrations");

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // loads env variables from .env
    dotenv::dotenv().ok();
    let database_url =
        std::env::var("DATABASE_URL").expect("DATABASE_URL must be set in the environment");

    // init tracing subscriber
    let tracing = tracing_subscriber::fmt()
        .with_timer(tracing_subscriber::fmt::time::UtcTime::rfc_3339())
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env());

    if cfg!(debug_assertions) {
        tracing.pretty().init();
    } else {
        tracing.json().init();
    }

    let pool = PgPoolOptions::new()
        .max_connections(1) // ollama local is quite slow
        .acquire_slow_threshold(Duration::from_secs(10))
        .connect(&database_url)
        .await
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

    // Migrating the database
    // MIGRATIONS.run(&pool).await.unwrap();

    let testimonial_repository =
        api_lib::testimonial_repository::PostgresTestimonialRepository::new(pool.clone());
    let testimonial_repository = actix_web::web::Data::new(testimonial_repository);

    let metric_repository = api_lib::metric_repository::PostgresMetricRepository::new(pool.clone());
    let metric_repository = actix_web::web::Data::new(metric_repository);

    let user_repository = api_lib::user_repository::PostgresUserRepository::new(pool.clone());
    let user_repository = actix_web::web::Data::new(user_repository);

    let insight_repository =
        api_lib::insight_repository::PostgresInsightRepository::new(pool.clone());
    let insight_repository = actix_web::web::Data::new(insight_repository);

    tracing::info!("Database connection pool created successfully!");

    // building address
    let port = std::env::var("PORT").unwrap_or("8000".to_string());
    let address = format!("127.0.0.1:{}", port);

    // static files
    let static_folder = std::env::var("STATIC_FOLDER").unwrap_or("static".to_string());

    HttpServer::new(move || {
        // CORS
        let cors = Cors::permissive();

        App::new()
            .wrap(cors)
            .service(
                web::scope("/api")
                    .app_data(testimonial_repository.clone())
                    .app_data(metric_repository.clone())
                    .app_data(user_repository.clone())
                    .app_data(insight_repository.clone())
                    .configure(api_lib::health::service)
                    .configure(
                        api_lib::v1::testimonial::service::<
                            api_lib::testimonial_repository::PostgresTestimonialRepository,
                        >,
                    )
                    .configure(
                        api_lib::v1::user::service::<
                            api_lib::user_repository::PostgresUserRepository,
                        >,
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
                Files::new("/", &static_folder)
                    .show_files_listing()
                    .index_file("index.html"),
            )
    })
    .bind(&address)
    .unwrap_or_else(|err| {
        panic!(
            "🔥🔥🔥 Couldn't start the server in port {}: {:?}",
            port, err
        )
    })
    .run()
    .await
}
