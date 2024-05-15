use reqwest::Client;

use crate::common::TestServer;

mod common;

#[tokio::test]
async fn health_check_works() {
    // Arrange
    let server = TestServer::run().await;

    // Act
    let response = Client::new()
        .get(&server.url("health_check"))
        .send()
        .await
        .expect("Failed to executed request");

    // Assert
    assert_eq!(204, response.status());
    assert_eq!(0, response.content_length().unwrap());
}
