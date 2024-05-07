use std::net::TcpListener;

use reqwest::Client;

use server::app::run_without_rabbit_mq;

#[tokio::test]
async fn health_check_works() {
    // Arrange
    let addr = spawn_app().await;
    let client = Client::new();

    // Act
    let response = client
        .get(&format!("{}/v1/health_check", addr))
        .send()
        .await
        .expect("Failed to executed request");

    // Assert
    assert_eq!(204, response.status());
    assert_eq!(0, response.content_length().unwrap());
}

async fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let addr = format!("http://{}", listener.local_addr().unwrap());
    let server = run_without_rabbit_mq(listener).unwrap();

    #[allow(clippy::let_underscore_future)]
    let _ = tokio::spawn(server);

    addr
}
