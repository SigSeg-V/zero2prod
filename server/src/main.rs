use std::net::TcpListener;

use zero2prod;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to port");
    zero2prod::run(listener)?.await
}


