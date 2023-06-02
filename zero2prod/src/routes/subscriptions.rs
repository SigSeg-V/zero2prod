use crate::db_settings::DbPool;
use crate::models::Subscriber;
use crate::schema;
use actix_web::{web, HttpResponse, Responder};
use uuid::Uuid;

use chrono;
use diesel::prelude::*;

pub async fn subscribe(
    form: web::Form<SubscribeForm>,
    connection: web::Data<DbPool>,
) -> actix_web::Result<impl Responder> {
    let mut connection = connection.get().expect("Failed to POST form.");
    let new_user = Subscriber {
        id: Uuid::new_v4(),
        email: form.email.clone(),
        name: form.name.clone(),
        subscribed_at: chrono::offset::Utc::now(),
    };
    diesel::insert_into(schema::subscriptions::table)
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
