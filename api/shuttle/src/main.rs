use actix_files::Files;
use actix_web::web::{self, ServiceConfig};
use shuttle_actix_web::ShuttleActixWeb;
use shuttle_runtime::CustomError;
use sqlx::migrate::Migrator;
use sqlx::postgres::PgPoolOptions;

pub static MIGRATIONS: Migrator = sqlx::migrate!("../migrations");

#[shuttle_runtime::main]
async fn server() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect("postgresql://postgres:postgres@localhost:5432/rep-star")
        // .connect("postgresql://tsdbadmin:u1ttq9i4o6ex24db@bhrhgjuo9r.m19kjwh83w.tsdb.cloud.timescale.com:39098/tsdb")
        .await
        .map_err(CustomError::new)?;

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

    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(
            web::scope("/api")
                .app_data(testimonial_repository)
                .app_data(metric_repository)
                .app_data(user_repository)
                .app_data(insight_repository)
                .configure(api_lib::health::service)
                .configure(
                    api_lib::v1::testimonial::service::<
                        api_lib::testimonial_repository::PostgresTestimonialRepository,
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
