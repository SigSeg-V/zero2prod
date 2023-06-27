use actix_web::{dev::Server, web, App, HttpServer};
use std::net::TcpListener;

use crate::routes;

use sea_orm::{DatabaseConnection};

pub fn run(listener: TcpListener, connection: DatabaseConnection) -> Result<Server, std::io::Error> {
    let conn = web::Data::new(connection);

    let server = HttpServer::new(move || {
        App::new()
            .route(
                "/health/check",
                web::get().to(routes::health_check::health_checker),
            )
            .route(
                "subscriptions",
                web::post().to(routes::subscriptions::subscribe),
            )
            .app_data(conn.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}
