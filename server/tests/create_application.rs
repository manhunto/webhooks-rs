use reqwest::Client;
use serde_json::{json, Value};

use server::types::ApplicationId;

use crate::common::TestEnvironment;

mod common;

#[tokio::test]
async fn application_is_created() {
    // Arrange
    let server = TestEnvironment::new().await.server().await;

    // Act
    let response = Client::new()
        .post(&server.url("application"))
        .json(&json!({
          "name": "Dummy application"
        }))
        .send()
        .await
        .expect("Failed to executed request");

    // Assert
    assert_eq!(201, response.status());

    let body = response.json::<Value>().await.unwrap();
    assert_eq!("Dummy application", body["name"].as_str().unwrap());

    let id = ApplicationId::try_from(body["id"].as_str().unwrap().to_string())
        .expect("Invalid application id");

    let app = server
        .storage()
        .applications
        .get(&id)
        .await
        .expect("Application was not created");

    assert_eq!("Dummy application", app.name);
}
