use actix_web::web::{Data, Json, Path};
use actix_web::{HttpResponse, Responder, Result};
use log::debug;

use crate::amqp::Publisher;
use crate::cmd::{AsyncMessage, SentMessage};
use crate::configuration::domain::{Endpoint, Topic};
use crate::error::ResponseError;
use crate::events::domain::{Event, Message, Payload};
use crate::events::models::{CreateEventRequest, CreateEventResponse};
use crate::storage::Storage;
use crate::time::Clock;
use crate::types::ApplicationId;

pub async fn create_event_handler(
    storage: Data<Storage>,
    dispatcher: Data<Publisher>,
    request: Json<CreateEventRequest>,
    path: Path<String>,
) -> Result<impl Responder, ResponseError> {
    let app_id = ApplicationId::try_from(path.into_inner())?;
    let app = storage.applications.get(&app_id).await?;
    let topic = Topic::new(request.topic.clone())?;
    let clock = Clock::chrono();
    let event = Event::new(
        app.id,
        Payload::from(request.payload.clone()),
        topic.clone(),
        &clock,
    );

    storage.events.save(event.clone()).await;

    debug!("Message created: {:?}", event,);

    let endpoints: Vec<Endpoint> = storage.endpoints.for_topic(&app_id, &event.topic).await;
    let endpoints_count = endpoints.len();

    let active_endpoints: Vec<Endpoint> =
        endpoints.into_iter().filter(|en| en.is_active()).collect();

    debug!(
        "in app {} - {} ({}) endpoints found for event {}",
        event.app_id,
        active_endpoints.len(),
        endpoints_count,
        event.id
    );

    for endpoint in active_endpoints {
        debug!("{} sending to {}", event.id, endpoint.url);

        let msg = Message::from((event.clone(), endpoint.clone()));

        storage.messages.save(msg.clone());

        let cmd = SentMessage::new(msg.id);
        let message = AsyncMessage::SentMessage(cmd);

        dispatcher.publish(message).await;

        debug!("Message {} published on the queue", msg.id)
    }

    Ok(HttpResponse::Ok().json(CreateEventResponse::from(event)))
}
