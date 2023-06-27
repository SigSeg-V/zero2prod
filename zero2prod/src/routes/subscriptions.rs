use actix_web::{web, HttpResponse, Responder};
use sea_orm::DatabaseConnection;
use uuid::Uuid;
use chrono;
use crate::entities;

pub async fn subscribe(
    form: web::Form<SubscribeForm>,
    connection: web::Data<DatabaseConnection>,
) -> actix_web::Result<impl Responder> {
    let new_user = entities::user::ActiveModel {
        name: sea_orm::ActiveValue::Set(form.name),
        email: sea_orm::ActiveValue::Set(form.email),
        created_at: chrono::NaiveDate,
    };

    diesel::insert_into(subscriptions::table)
        .values(new_user)
        .execute(&mut connection)
        .expect("Failed to POST form.");

    Ok(HttpResponse::Ok().finish())
}

#[derive(serde::Deserialize)]
pub struct SubscribeForm {
    pub email: String,
    pub name: String,
}
