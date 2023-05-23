use diesel::{PgConnection, Connection};
use dotenvy;

fn init_env() {
    dotenvy::dotenv().expect("Failed to init env var.");
    match dotenvy::from_filename("prod.env").ok(){
        None => println!("Warning: could not find prod.env"),
        _ => ()
    }

}

pub fn conn_str() -> String {
    init_env();
    std::env::var(r"DATABASE_URL").expect("DB URL not found")
}

pub fn establish_connection() -> PgConnection {
    let conn = conn_str();
    PgConnection::establish(conn.as_str())
       .unwrap_or_else(|_| panic!("Unable to connect to db: {}", conn))
}