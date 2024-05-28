use std::time::Duration;

use mockito::Matcher::Json;
use mockito::Server;
use reqwest::Client;
use serde_json::{json, Value};
use tokio::time::sleep;

use server::configuration::domain::Topic;
use server::types::EventId;

use crate::common::{run_test_server_and_dispatcher, Given, TestEnvironment};

mod common;

#[tokio::test]
async fn event_is_created_and_dispatched() {
    // Arrange
    let server = run_test_server_and_dispatcher!();

    let mut destination_server = Server::new_async().await;
    let mock = destination_server
        .mock("POST", "/some_endpoint")
        .match_body(Json(json!({
           "nested": {
              "foo": "bar"
           }
        })))
        .with_status(201)
        .create_async()
        .await;

    let topic = "contact.created";
    let (app_id, _) = Given::from(&server)
        .endpoint_with_app(
            &format!("{}/some_endpoint", destination_server.url()),
            vec![topic],
        )
        .await;
    let payload = json!({
       "nested": {
          "foo": "bar"
       }
    });

    // Act
    let response = Client::new()
        .post(&server.url(&format!("application/{}/event", app_id)))
        .json(&json!({
          "topic": topic,
          "payload": payload
        }))
        .send()
        .await
        .expect("Failed to executed request");

    // Assert
    assert_eq!(200, response.status());
    let body = response.json::<Value>().await.unwrap();
    let id = EventId::try_from(body["id"].as_str().unwrap().to_string()).expect("Invalid event id");

    let event = server
        .storage()
        .events
        .get(id)
        .await
        .expect("Event wasn't persisted");

    assert_eq!(id, event.id);
    assert_eq!(
        serde_json::to_value(payload).unwrap(),
        serde_json::to_value(event.payload.clone()).unwrap()
    );
    assert_eq!(Topic::try_from("contact.created").unwrap(), event.topic);
    println!("{:?}", event);

    sleep(Duration::from_millis(10)).await; // todo how to remove sleep? consume it once?
    mock.assert_async().await;
}
