use reqwest::Client;
use serde_json::{json, Value};
use url::Url;

use server::configuration::domain::{EndpointStatus, TopicsList};
use server::types::EndpointId;

use crate::common::{Given, TestEnvironment};

mod common;

#[tokio::test]
async fn endpoint_is_created() {
    // Arrange
    let server = TestEnvironment::new().await.server().await;
    let app_id = Given::from(&server).app().await;

    // Act
    let response = Client::new()
        .post(&server.url(&format!("application/{}/endpoint", app_id)))
        .json(&json!({
          "url": "http://localhost:8080",
          "topics": [
            "contact.updated",
            "contact.created"
          ]
        }))
        .send()
        .await
        .expect("Failed to executed request");

    // Assert
    assert_eq!(201, response.status());

    let body = response.json::<Value>().await.unwrap();

    let id = EndpointId::try_from(body["id"].as_str().unwrap().to_string())
        .expect("Invalid endpoint id");

    let endpoint = server
        .storage()
        .endpoints
        .get(&id)
        .await
        .expect("Endpoint not found");

    assert_eq!(
        TopicsList::from(vec!["contact.updated", "contact.created"]),
        endpoint.topics
    );
    assert_eq!(EndpointStatus::Initial, endpoint.status);
    assert_eq!(app_id, endpoint.app_id);
    assert_eq!(Url::parse("http://localhost:8080").unwrap(), endpoint.url);
}
