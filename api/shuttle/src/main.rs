use actix_web::web::ServiceConfig;
use api_lib::health::{hello_world, version};
use shuttle_actix_web::ShuttleActixWeb;
use shuttle_runtime::CustomError;
use sqlx::postgres::PgPoolOptions;

#[shuttle_runtime::main]
async fn actix_web() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    // Create a connection pool
    //  for MySQL/MariaDB, use MySqlPoolOptions::new()
    //  for SQLite, use SqlitePoolOptions::new()
    //  etc.
    let pool = PgPoolOptions::new()
        .max_connections(1)
        .connect("postgresql://tsdbadmin:u1ttq9i4o6ex24db@bhrhgjuo9r.m19kjwh83w.tsdb.cloud.timescale.com:39098/tsdb")
        .await
        .map_err(CustomError::new)?;

    let pool = actix_web::web::Data::new(pool);

    tracing::info!("Database connection pool created successfully!");

    let config = move |cfg: &mut ServiceConfig| {
        cfg.app_data(pool).service(hello_world).service(version);
    };

    // initialize the database if not already initialized
    // pool.execute(include_str!("../../db/schema.sql"))
    //     .await
    //     .map_err(CustomError::new)?;

    Ok(config.into())
}
