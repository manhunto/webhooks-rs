use crate::configuration::domain::{ApplicationId, Topic};
use crate::error::ResponseError;
use crate::events::domain::{Message, Payload};
use crate::events::models::CreateMessageRequest;
use crate::storage::Storage;
use actix_web::web::{Data, Json, Path};
use actix_web::{HttpResponse, Responder, Result};
use lapin::options::{BasicPublishOptions, QueueDeclareOptions};
use lapin::publisher_confirm::Confirmation;
use lapin::types::FieldTable;
use lapin::{BasicProperties, Connection, ConnectionProperties};
use log::{debug, info};

pub async fn create_message_handler(
    storage: Data<Storage>,
    request: Json<CreateMessageRequest>,
    path: Path<String>,
) -> Result<impl Responder, ResponseError> {
    let app_id = ApplicationId::try_from(path.into_inner()).unwrap();
    let app = storage.applications.get(&app_id)?;
    let topic = Topic::new(request.topic.clone())?;
    let msg = Message::new(
        app.id,
        Payload::from(request.payload.clone()),
        topic.clone(),
    );

    storage.messages.save(msg.clone());

    debug!(
        "Message created: {:?}, count: {}",
        msg,
        storage.messages.count()
    );

    let addr = "amqp://guest:guest@localhost:5672";

    let conn = Connection::connect(addr, ConnectionProperties::default())
        .await
        .unwrap();

    info!("connected established with rabbitmq");

    let channel = conn.create_channel().await.unwrap();

    let queue = channel
        .queue_declare(
            "sent_message",
            QueueDeclareOptions::default(),
            FieldTable::default(),
        )
        .await
        .unwrap();

    debug!("queue declared {:?}", queue);

    let endpoints = storage.endpoints.for_topic(&app_id, &topic);
    debug!("{} endpoints found for message {}", endpoints.len(), msg.id);

    for endpoint in endpoints {
        debug!("{} sending to {}", msg.id, endpoint.url);

        let payload_as_str = msg.payload.to_string();
        let payload = payload_as_str.as_bytes();

        let confirm = channel
            .basic_publish(
                "",
                "sent_message",
                BasicPublishOptions::default(),
                payload,
                BasicProperties::default(),
            )
            .await
            .unwrap()
            .await
            .unwrap();

        assert_eq!(confirm, Confirmation::NotRequested);

        debug!("Message published on the queue")
    }

    Ok(HttpResponse::Created())
}
