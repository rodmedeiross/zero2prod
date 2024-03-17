use actix_web::dev::Server;
#[allow(unused_imports)]
use actix_web::{web, App, HttpRequest, HttpResponse, HttpResponseBuilder, HttpServer, Responder};
use std::net::TcpListener;

fn config_handlers(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/healthz").route(web::get().to(health_check)));
}

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    // HttpServer::new(|| App::new().route("/healthz", web::get().to(health_check)))
    let server = HttpServer::new(|| App::new().configure(config_handlers))
        .listen(listener)?
        .run();

    Ok(server)
}
