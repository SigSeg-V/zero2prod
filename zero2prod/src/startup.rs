use actix_web::{ HttpServer, App, web, dev::Server};
use std::net::TcpListener;

use crate::routes;

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
        .route("/health/check", web::get().to(routes::health_check::health_checker))
        .route("subscriptions", web::post().to(routes::subscriptions::subscribe))
    })
    .listen(listener)?
    .run();
    Ok(server)
}