use reqwest::Client;
use serde_json::json;

use crate::common::{Given, TestEnvironment};

mod common;

#[tokio::test]
async fn event_is_created_and_dispatched() {
    // Arrange
    let environment = TestEnvironment::new().await;
    let server = environment.server().run().await;

    environment.dispatcher().run().await;

    let topic = "contact.created";
    let (app_id, _) = Given::from(&server)
        .endpoint_with_app("http://localhost:8080", vec![topic])
        .await;

    // Act
    let response = Client::new()
        .post(&server.url(&format!("application/{}/event", app_id)))
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
