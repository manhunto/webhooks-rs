use reqwest::Client;
use serde_json::json;

use crate::common::{Given, TestServer};

mod common;

#[tokio::test]
async fn message_is_created_and_dispatched() {
    // Arrange
    let server = TestServer::run().await;
    let topic = "contact.created";
    let (app_id, _) = Given::from(&server)
        .endpoint_with_app("http://localhost:8080", vec![topic])
        .await;

    // Act
    let response = Client::new()
        .post(&server.url(&format!("application/{}/message", app_id)))
        .json(&json!({
          "topic": topic,
          "payload": {
             "nested": {
                "foo": "bar"
             }
          }
        }))
        .send()
        .await
        .expect("Failed to executed request");

    // Assert
    assert_eq!(201, response.status());

    // todo: assert, was message dispatched to rabbit? or maybe consume it.. and try to check if was dispatched on server
}
