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
        api_lib::testimonial_repository::PostgresTestimonialRepository::new(pool);

    let testimonial_repository = actix_web::web::Data::new(testimonial_repository);

    tracing::info!("Database connection pool created successfully!");

    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(
            web::scope("/api")
                .app_data(testimonial_repository)
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
