use std::net::TcpListener;

use actix_web::dev::Server;
use actix_web::web::Data;
use actix_web::{web, App, HttpServer};

use sqlx::PgPool;
use tracing_actix_web::TracingLogger;
use crate::background_process::background_worker;

use crate::routes::{get_all_jobs, health_check};

pub fn run(listener: TcpListener, db_pool: PgPool) -> Result<Server, std::io::Error> {
    tokio::task::spawn(background_worker(db_pool.clone()));
    let db_pool = Data::new(db_pool);

    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .route("/health_check", web::get().to(health_check))
            .route("/get_jobs", web::get().to(get_all_jobs))
            .app_data(db_pool.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}
// Create job form should return an OK 200 with the object
