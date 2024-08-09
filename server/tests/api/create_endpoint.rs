use reqwest::Client;
use serde_json::{json, Value};
use url::Url;

use server::configuration::domain::{EndpointStatus, TopicsList};
use server::types::{ApplicationId, EndpointId};

use crate::common::{run_test_server, Given, TestEnvironment};

#[tokio::test]
async fn endpoint_is_created() {
    // Arrange
    let server = run_test_server!();
    let app_id = Given::from(&server).app().await;

    // Act
    let response = Client::new()
        .post(server.url(&format!("application/{}/endpoint", app_id)))
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

#[tokio::test]
async fn validation() {
    // Arrange
    let server = run_test_server!();
    let app_id = Given::from(&server).app().await;

    let test_cases = vec![
        (
            ApplicationId::new(),
            json!({"url": "http://localhost", "topics": ["contact.created"]}),
            404,
            json!({"error": "Entity not found", "messages": []}), // fixme: change for something like "Application not found"
        ),
        (
            app_id,
            json!({"url": "", "topics": ["contact.created"]}),
            400,
            json!({"error": "Validation errors", "messages": ["Url should be valid"]}),
        ),
        (
            app_id,
            json!({"url": "invalid-url", "topics": ["contact.created"]}),
            400,
            json!({"error": "Validation errors", "messages": ["Url should be valid"]}),
        ),
        (
            app_id,
            json!({"url": "http://localhost", "topics": []}),
            400,
            json!({"error": "Validation errors", "messages": ["Should be at leas one topic"]}),
        ),
        (
            app_id,
            json!({"url": "http://localhost", "topics": ["foo bar"]}),
            400,
            json!({"error": "Validation errors", "messages": ["'foo bar' is invalid topic name"]}),
        ),
        (
            app_id,
            json!({"url": "http://localhost", "topics": ["foo.bar", "bar baz"]}),
            400,
            json!({"error": "Validation errors", "messages": ["'bar baz' is invalid topic name"]}),
        ),
        // (
        //     app_id,
        //     json!({"url": "http://localhost", "topics": ["foo bar", "bar baz"]}),
        //     400,
        //     json!({"error": "Validation errors", "messages": ["'foo bar' is invalid topic name", "'bar baz' is invalid topic name"]}),
        // ),
    ];

    for test_case in test_cases {
        // Act
        let response = Client::new()
            .post(server.url(&format!("application/{}/endpoint", test_case.0)))
            .json(&test_case.1)
            .send()
            .await
            .expect("Failed to executed request");

        // Assert
        assert_eq!(test_case.2, response.status());
        assert_eq!(test_case.3, response.json::<Value>().await.unwrap());
    }
}
