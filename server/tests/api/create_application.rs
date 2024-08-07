use reqwest::Client;
use serde_json::{json, Value};

use server::types::ApplicationId;

use crate::common::{run_test_server, TestEnvironment};

#[tokio::test]
async fn application_is_created() {
    // Arrange
    let server = run_test_server!();

    // Act
    let response = Client::new()
        .post(server.url("application"))
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

#[tokio::test]
async fn application_names_can_be_without_space() {
    // Arrange
    let server = run_test_server!();

    // Act
    let response = Client::new()
        .post(server.url("application"))
        .json(&json!({
          "name": "test"
        }))
        .send()
        .await
        .expect("Failed to executed request");

    // Assert
    assert_eq!(201, response.status());
}

#[tokio::test]
async fn validation() {
    // Arrange
    let server = run_test_server!();

    let test_cases = vec![
        (
            json!({"name": ""}),
            json!({"error": "Validation errors", "messages": ["Name cannot be empty"]}),
        ),
        (
            json!({"name": "  "}),
            json!({"error": "Validation errors", "messages": ["Name cannot be empty"]}),
        ),
    ];

    for test_case in test_cases {
        // Act
        let response = Client::new()
            .post(server.url("application"))
            .json(&test_case.0)
            .send()
            .await
            .expect("Failed to executed request");

        // Assert
        assert_eq!(400, response.status());
        assert_eq!(test_case.1, response.json::<Value>().await.unwrap());
    }
}
