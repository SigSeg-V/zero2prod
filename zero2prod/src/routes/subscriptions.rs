use crate::models::Subscriber;
use crate::{db_settings::DbPool, schema::subscriptions};
use actix_web::{web, HttpResponse, Responder};
use diesel::{PgConnection, RunQueryDsl};
use diesel_async::AsyncConnection;
use uuid::Uuid;

use chrono;

pub async fn subscribe(
    form: web::Form<SubscribeForm>,
    connection: web::Data<DbPool>,
) -> actix_web::Result<impl Responder> {
    let mut connection = &mut connection.get().unwrap();
    if cfg!(test) {
        connection.begin_test_transaction().unwrap();
    }
    let new_user = Subscriber {
        id: Uuid::new_v4(),
        email: form.email.clone(),
        name: form.name.clone(),
        subscribed_at: chrono::offset::Utc::now(),
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
