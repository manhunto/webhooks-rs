use crate::configuration::domain::ApplicationId;
use crate::events::domain::{Message, Payload};
use crate::events::models::CreateMessageRequest;
use crate::storage::Storage;
use actix_web::web::{Data, Json, Path};
use actix_web::{HttpResponse, Responder};
use log::debug;

pub async fn create_message_handler(
    storage: Data<Storage>,
    request: Json<CreateMessageRequest>,
    path: Path<String>,
) -> impl Responder {
    let app_id = path.into_inner();
    // todo check if app exists

    let msg = Message::new(
        ApplicationId::try_from(app_id).unwrap(),
        Payload::from(request.payload.clone()),
    );

    storage.messages.save(msg.clone());

    debug!(
        "Message created: {:?}, count: {}",
        msg,
        storage.messages.count()
    );

    // todo fetch all endpoints for app
    // filter by topics
    // todo dispatch

    HttpResponse::Created()
}
