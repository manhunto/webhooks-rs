use reqwest::Client;

use server::configuration::domain::EndpointStatus;

use crate::common::{run_test_server, Given, TestEnvironment};

const FAKE_URL: &str = "http://localhost:0";
const FAKE_TOPIC: &str = "contact.created";

#[tokio::test]
async fn endpoint_can_be_disabled() {
    // Arrange
    let server = run_test_server!();
    let (app_id, endpoint_id) = Given::from(&server)
        .endpoint_with_app(FAKE_URL, vec![FAKE_TOPIC])
        .await;

    // Act
    let response = Client::new()
        .post(server.url(&format!(
            "application/{}/endpoint/{}/disable",
            app_id, endpoint_id
        )))
        .send()
        .await
        .expect("Failed to executed request");

    // Assert
    assert_eq!(204, response.status());

    let endpoint = server
        .storage()
        .endpoints
        .get(&endpoint_id)
        .await
        .expect("Endpoint doesn't exist");

    assert_eq!(EndpointStatus::DisabledManually, endpoint.status);
}

#[tokio::test]
async fn endpoint_can_be_enabled() {
    // Arrange
    let server = run_test_server!();
    let given = Given::from(&server);
    let (app_id, endpoint_id) = given.endpoint_with_app(FAKE_URL, vec![FAKE_TOPIC]).await;

    given.disable_endpoint(&app_id, &endpoint_id).await;

    // Act
    let response = Client::new()
        .post(server.url(&format!(
            "application/{}/endpoint/{}/enable",
            app_id, endpoint_id
        )))
        .send()
        .await
        .expect("Failed to executed request");

    // Assert
    assert_eq!(204, response.status());

    let endpoint = server
        .storage()
        .endpoints
        .get(&endpoint_id)
        .await
        .expect("Endpoint doesn't exist");

    assert_eq!(EndpointStatus::EnabledManually, endpoint.status);
}
