use actix_web::{ HttpRequest, Responder, HttpResponse, HttpServer, App, web, dev::Server};
use std::net::TcpListener;

pub async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello, {name}")
}

pub async fn health_checker() -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[derive(serde::Deserialize)]
pub struct SubscribeForm {
    email: String,
    name: String,
}

pub async fn subscribe(_form: web::Form<SubscribeForm>) -> HttpResponse {
    HttpResponse::Ok().finish()
}


pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
        .route("/", web::get().to(greet))
        .route("/{name}", web::get().to(greet))
        .route("/health/check", web::get().to(health_checker))
        .route("subscriptions", web::post().to(subscribe))
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

    #[tokio::test]
    async fn subscribe_ret_200_on_valid_form() {
        // bind addr
        let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to port");

        // spawn app and client inst.
        let addr = spawn_app(listener);
        let client = reqwest::Client::new();

        // build form for submission
        let form = "name=ben%20dover&email=bendover%40hotmail.com";

        // POST form
        let resp = client.post(format!("{}/subscriptions", addr))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(form)
        .send()
        .await
        .expect("Failed to POST form.");

        assert_eq!(resp.status().as_u16(), 200)
    }

    #[tokio::test]
    async fn subscribe_ret_400_on_missing_item() {
        // bind addr
        let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to port");

        // spawn app and client inst.
        let addr = spawn_app(listener);
        let client = reqwest::Client::new();

        // build form for submission
        let tests = vec![
            ("name=ben%20dover", "Missing email"),
            ("email=bendover%40hotmail.com", "Missing name"),
            ("", "Missing both email and name"),
        ];

        for (test, msg) in tests {
            // POST form
            let resp = client.post(format!("{}/subscriptions", addr))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(test)
        .send()
        .await
        .expect("Failed to POST form.");

            assert_eq!(resp.status().as_u16(), 400, "{} test failed.", msg)
        }


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