use std::net::TcpListener;

use reqwest::Client;

#[tokio::test]
async fn health_check_works() {
    let address = spawn_app();

    let client = Client::new();
    let response = client
        .get(format!("{address}/health_check"))
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(response.status(), 200);
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind address.");
    let port = listener.local_addr().unwrap().port();
    let server = zero2prod::run(listener).expect("Failed to bind address.");
    tokio::spawn(server);
    format!("http://127.0.0.1:{port}")
}
