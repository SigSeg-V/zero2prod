use std::net::TcpListener;

use zero2prod;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to port");
    let pool = zero2prod::db_settings::initialize_db_pool().await;
    zero2prod::run(listener, pool)?.await;
    Ok(())
}
