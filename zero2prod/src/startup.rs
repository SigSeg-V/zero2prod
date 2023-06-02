use actix_web::{ HttpServer, App, web, dev::Server};
use std::net::TcpListener;
use crate::db_settings::DbPool;

use crate::routes;

pub fn run(listener: TcpListener, connection: DbPool) -> Result<Server, std::io::Error> {

    let connection = web::Data::new(connection);

    let server = HttpServer::new(move || {
        App::new()
        .route("/health/check", web::get().to(routes::health_check::health_checker))
        .route("subscriptions", web::post().to(routes::subscriptions::subscribe))
        .app_data(connection.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}