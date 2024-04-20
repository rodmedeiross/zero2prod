use crate::routes::{health_check, subscribe};
use actix_web::{dev::Server, web, App, HttpServer};
use sqlx::PgPool;
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;

fn config_handlers(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/healthz").route(web::get().to(health_check)))
        .service(web::resource("/subscriptions").route(web::post().to(subscribe)));
}

pub fn run(listener: TcpListener, connection_pool: PgPool) -> Result<Server, std::io::Error> {
    // HttpServer::new(|| App::new().route("/healthz", web::get().to(health_check)))
    let connection = web::Data::new(connection_pool);
    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .configure(config_handlers)
            .app_data(connection.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
