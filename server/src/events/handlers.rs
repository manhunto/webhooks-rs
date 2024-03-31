use actix_web::{HttpResponse, Responder, Result};
use actix_web::web::{Data, Json, Path};
use log::debug;

use crate::amqp::Publisher;
use crate::cmd::{AsyncMessage, SentMessage};
use crate::configuration::domain::{ApplicationId, Topic};
use crate::error::ResponseError;
use crate::events::domain::{Message, Payload};
use crate::events::models::CreateMessageRequest;
use crate::storage::Storage;

pub async fn create_message_handler(
    storage: Data<Storage>,
    dispatcher: Data<Publisher>,
    request: Json<CreateMessageRequest>,
    path: Path<String>,
) -> Result<impl Responder, ResponseError> {
    let app_id = ApplicationId::try_from(path.into_inner()).unwrap();
    let app = storage.applications.get(&app_id)?;
    let topic = Topic::new(request.topic.clone())?;
    let msg = Message::new(
        app.id,
        Payload::from(request.payload.to_string()),
        topic.clone(),
    );

    storage.messages.save(msg.clone());

    debug!(
        "Message created: {:?}, count: {}",
        msg,
        storage.messages.count()
    );

    let endpoints = storage.endpoints.for_topic(&app_id, &msg.topic);
    debug!(
        "in app {} - {} endpoints found for message {}",
        msg.app_id,
        endpoints.len(),
        msg.id
    );

    for endpoint in endpoints {
        debug!("{} sending to {}", msg.id, endpoint.url);

        let cmd = SentMessage::new(msg.payload.clone(), endpoint.url, msg.id.clone());
        let message = AsyncMessage::SentMessage(cmd);

        dispatcher.publish(message).await;

        debug!("Message published on the queue")
    }

    Ok(HttpResponse::Created())
}
