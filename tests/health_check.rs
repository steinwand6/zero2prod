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

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_from_data() {
    let address = spawn_app();
    let client = Client::new();

    let test_case = "name=le%20guin&email=ursula_le_guin%40gmail.com";

    let response = client
        .post(format!("{address}/subscriptions"))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(test_case)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response.status());
}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    let address = spawn_app();
    let client = Client::new();

    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(format!("{address}/subscriptions"))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request.");

        assert_eq!(
            400,
            response.status(),
            "The API did not fail with 400 Bad Request when the payload was {error_message}."
        )
    }
}

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind address.");
    let port = listener.local_addr().unwrap().port();
    let server = zero2prod::startup::run(listener).expect("Failed to bind address.");
    tokio::spawn(server);
    format!("http://127.0.0.1:{port}")
}
