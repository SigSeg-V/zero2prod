use actix_web::HttpResponse;

pub async fn health_checker() -> HttpResponse {
    HttpResponse::Ok().finish()
}