use reqwest::Client;

use crate::common::spawn_app;

mod common;

#[tokio::test]
async fn health_check_works() {
    // Arrange
    let addr = spawn_app();

    // Act
    let response = Client::new()
        .get(&format!("{}/v1/health_check", addr))
        .send()
        .await
        .expect("Failed to executed request");

    // Assert
    assert_eq!(204, response.status());
    assert_eq!(0, response.content_length().unwrap());
}
