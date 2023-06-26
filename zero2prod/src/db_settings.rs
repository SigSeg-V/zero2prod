use std::time::Duration;

use diesel::{r2d2::ConnectionManager, r2d2::Pool, Connection, PgConnection};
use dotenvy;

fn init_env() {
    dotenvy::dotenv().expect("Failed to init env var.");
    match dotenvy::from_filename("prod.env").ok() {
        None => println!("Warning: could not find prod.env"),
        _ => (),
    }
}

pub fn conn_str() -> String {
    init_env();
    std::env::var(r"DATABASE_URL").expect("DB URL not found")
}

pub fn conn_str_custom_db_name(db_name: String) -> String {
    init_env();
    format!(
        "postgres://{}:{}@{}:{}/{}",
        std::env::var(r"DB_USER").expect("DB User not found"),
        std::env::var(r"DB_PASSWD").expect("DB Passwd not found"),
        std::env::var(r"DB_HOST").expect("DB Host not found"),
        std::env::var(r"DB_PORT").expect("DB Port not found"),
        db_name
    )
}

pub fn establish_connection() -> PgConnection {
    let conn = conn_str();

    let mut conn = PgConnection::establish(conn.as_str())
        .unwrap_or_else(|_| panic!("Unable to connect to db: {}", conn));
    if cfg!(test) {
        conn.begin_test_transaction()
            .expect("Failed to start transaction");
    }
    conn
}

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

pub fn initialize_db_pool(pool_size: u32) -> DbPool {
    let connection_str = conn_str();
    let manager = ConnectionManager::<PgConnection>::new(connection_str);
    Pool::builder()
        .max_size(pool_size)
        .min_idle(Some(std::cmp::min(5, pool_size)))
        .max_lifetime(Some(Duration::from_secs(60 * 60 * 24)))
        .idle_timeout(Some(Duration::from_secs(60 * 2)))
        .build(manager)
        .expect("Failed to create pool.")
}
