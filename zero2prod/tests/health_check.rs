use std::net::TcpListener;

use zero2prod::{models::*, schema::subscriptions::dsl::*, run, db_settings::{self, DbPool}};
use diesel::{prelude::*, result::Error};

// Load a blank page with GET: 200
#[tokio::test]
async fn health_check_success() {
    // spawn app and client inst.
    let addr = spawn_app();
    let TestApp(addr, pool) = addr.await;
    let client = reqwest::Client::new();

    // SEND
    let resp = client
        .get(&format!("{}/health/check", addr))
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(resp.status().is_success(), true);
    assert_eq!(resp.content_length(), Some(0))
}

#[tokio::test]
async fn subscribe_ret_200_on_valid_form() {
    // spawn app and client inst.
    let addr = spawn_app();
    let TestApp(addr, pool) = addr.await;
    let client = reqwest::Client::new();
    let conn = &mut db_settings::establish_connection();

    conn.test_transaction::<_, Error, _>(|conn| {
        // build form for submission
        let form = "name=ben%20dover&email=bendover%40hotmail.com";

        // POST form
        let resp = client.post(format!("{}/subscriptions", addr))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(form)
            .send()
            .await
            .expect("Failed to POST form.");

        assert_eq!(resp.status().as_u16(), 200);

        let saved = subscriptions
                .filter(email.eq("bendover@hotmail.com".to_string()))
                .limit(1)
                .load::<Subscriber>(conn)
                .expect("Error loading users");

        assert_eq!(saved.is_empty(), false);
        for sub in saved {
            assert_eq!(sub.email, "bendover@hotmail.com");
            assert_eq!(sub.name, "ben dover");
        }
        Ok(())
    });
}

#[tokio::test]
async fn subscribe_ret_400_on_missing_item() {
    // spawn app and client inst.
    let addr = spawn_app();
    let client = reqwest::Client::new();

    // build form for submission
    let tests = vec![
            ("name=ben%20dover", "Missing email"),
            ("email=bendover%40hotmail.com", "Missing name"),
            ("", "Missing both email and name"),
        ];

    let TestApp(addr, pool) = addr.await;

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

struct TestApp(String, DbPool);

// returns address:port of the app
async fn spawn_app() -> TestApp {

    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to port");
    let port = listener.local_addr().unwrap().port();

    let db_name = uuid::Uuid::new_v4().to_string();
    let pool = db_settings::initialize_db_pool();

    let server = run(listener, pool.clone()).expect("Failed to bind address.");

    // spawn thread to run srvr so tests can run
    tokio::spawn(server);

    TestApp(format!(r"http://127.0.0.1:{}", port), pool)
}