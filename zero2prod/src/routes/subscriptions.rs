use actix_web::{HttpResponse, web};

pub async fn subscribe(_form: web::Form<SubscribeForm>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[derive(serde::Deserialize)]
pub struct SubscribeForm {
    email: String,
    name: String,
}
