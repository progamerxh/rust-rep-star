use actix_web::web::ServiceConfig;
use shuttle_actix_web::ShuttleActixWeb;
use shuttle_runtime::CustomError;
use sqlx::{postgres::PgPoolOptions, Executor};

#[shuttle_runtime::main]
async fn server() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    // Create a connection pool
    //  for MySQL/MariaDB, use MySqlPoolOptions::new()
    //  for SQLite, use SqlitePoolOptions::new()
    //  etc.
    let pool = PgPoolOptions::new()
        .max_connections(1)
        .connect("postgresql://postgres:postgres@localhost:5432/rep-star")
        // .connect("postgresql://tsdbadmin:u1ttq9i4o6ex24db@bhrhgjuo9r.m19kjwh83w.tsdb.cloud.timescale.com:39098/tsdb")
        .await
        .map_err(CustomError::new)?;

    // initialize the database if not already initialized
    pool.execute(include_str!("../../db/schema.sql"))
        .await
        .map_err(CustomError::new)?;

    let testimonial_repository =
        api_lib::testimonial_repository::PostgresTestimonialRepository::new(pool);

    let testimonial_repository: actix_web::web::Data<
        Box<dyn api_lib::testimonial_repository::TestimonialRepository>,
    > = actix_web::web::Data::new(Box::new(testimonial_repository));

    tracing::info!("Database connection pool created successfully!");

    let config = move |cfg: &mut ServiceConfig| {
        cfg.app_data(testimonial_repository)
            .configure(api_lib::health::service)
            .configure(api_lib::testimonials::service);
    };

    Ok(config.into())
}
