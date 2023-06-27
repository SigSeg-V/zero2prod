use std::time::Duration;

use dotenvy;
use sea_orm::{Database, ConnectOptions, DatabaseConnection, DbErr};

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

pub async fn initialize_db_pool(pool_size: u32) -> Result<DatabaseConnection, DbErr> {
    let mut opts = ConnectOptions::new(conn_str());
        opts.connect_timeout(Duration::from_secs(10))
        .acquire_timeout(Duration::from_secs(10))
        .idle_timeout(Duration::from_secs(10));
    Database::connect(opts).await
}
