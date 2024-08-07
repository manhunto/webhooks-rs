use reqwest::Client;

use crate::common::{run_test_server, TestEnvironment};

#[tokio::test]
async fn health_check_works() {
    // Arrange
    let server = run_test_server!();

    // Act
    let response = Client::new()
        .get(server.url("health_check"))
        .send()
        .await
        .expect("Failed to executed request");

    // Assert
    assert_eq!(204, response.status());
    assert_eq!(0, response.content_length().unwrap());
}
