use diesel::prelude::*;
use uuid::Uuid;
use chrono::DateTime;

#[derive(Queryable, Debug)]
pub struct Subscriber {
    pub id: Uuid,
    pub email: String,
    pub name: String,
    pub subscribed_at: DateTime<chrono::Utc>,
}