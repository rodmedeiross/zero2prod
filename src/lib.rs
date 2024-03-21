pub mod configuration;
pub mod routes;
pub mod startup;

use routes::health_check;
use routes::subscribe;

use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use std::net::TcpListener;

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
