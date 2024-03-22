use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use std::net::TcpListener;

use crate::routes::health_check;
use crate::routes::subscribe;

fn config_handlers(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/healthz").route(web::get().to(health_check)))
        .service(web::resource("/subscriptions").route(web::post().to(subscribe)));
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    // HttpServer::new(|| App::new().route("/healthz", web::get().to(health_check)))
    let server = HttpServer::new(|| App::new().configure(config_handlers))
        .listen(listener)?
        .run();

    Ok(server)
}
