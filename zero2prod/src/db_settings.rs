use diesel::{r2d2, Connection, PgConnection};
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
    PgConnection::establish(conn.as_str())
        .unwrap_or_else(|_| panic!("Unable to connect to db: {}", conn))
}

pub type DbPool = r2d2::Pool<r2d2::ConnectionManager<PgConnection>>;

pub fn initialize_db_pool() -> DbPool {
    let connection_str = conn_str();
    let manager = r2d2::ConnectionManager::<PgConnection>::new(connection_str);

    r2d2::Pool::builder()
        .build(manager)
        .expect("database URL should be valid path to Postgres DB")
}
