use diesel::prelude::*;
use uuid::Uuid;
use chrono::DateTime;
use crate::schema::subscriptions;

#[derive(Queryable, Selectable)]
#[diesel(table_name = subscriptions)]
pub struct Subscriber {
    pub id: Uuid,
    pub email: String,
    pub name: String,
    pub subscribed_at: DateTime<chrono::Utc>,
}