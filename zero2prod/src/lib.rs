use actix_web::{ HttpRequest, Responder, HttpResponse, HttpServer, App, web, dev::Server};
use std::net::TcpListener;

pub async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello, {name}")
}

pub async fn health_checker() -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
        .route("/", web::get().to(greet))
        .route("/{name}", web::get().to(greet))
        .route("/health/check", web::get().to(health_checker))
    })
    .listen(listener)?
    .run();
    Ok(server)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    
    // Load a blank page with GET: 200
    #[tokio::test]
    async fn health_check_success() {
        // bind addr
        let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to port");
        
        // spawn app and client inst.
        let addr = spawn_app(listener);
        let client = reqwest::Client::new();
        
        // SEND
        let resp = client
        .get(&format!("{addr}/health/check"))
        .send()
        .await
        .expect("Failed to execute request");
        
        assert_eq!(resp.status().is_success(), true);
        assert_eq!(resp.content_length(), Some(0))
    }

    // returns address:port of the app
    fn spawn_app(listener: TcpListener) -> String {
        let port = listener.local_addr().unwrap().port();
        let server = run(listener).expect("Failed to bind address.");
        
        // spawn thread to run srvr so tests can run
        tokio::spawn(server);
        
        format!(r"http://127.0.0.1:{}", port)
    }
}