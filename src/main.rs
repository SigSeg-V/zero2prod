use actix_web::{App, HttpServer, HttpRequest, Responder, web, HttpResponse};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
        .route("/", web::get().to(greet))
        .route("/{name}", web::get().to(greet))
        .route("/health/check", web::get().to(health_checker))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello, {name}")
}

async fn health_checker(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok()
}