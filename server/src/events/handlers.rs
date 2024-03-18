use crate::configuration::domain::ApplicationId;
use crate::error::ResponseError;
use crate::events::domain::{Message, Payload};
use crate::events::models::CreateMessageRequest;
use crate::storage::Storage;
use actix_web::web::{Data, Json, Path};
use actix_web::{HttpResponse, Responder, Result};
use log::debug;

pub async fn create_message_handler(
    storage: Data<Storage>,
    request: Json<CreateMessageRequest>,
    path: Path<String>,
) -> Result<impl Responder, ResponseError> {
    let app_id = ApplicationId::try_from(path.into_inner()).unwrap();
    let app = storage.applications.get(&app_id)?;

    let msg = Message::new(app.id, Payload::from(request.payload.clone()));

    storage.messages.save(msg.clone());

    debug!(
        "Message created: {:?}, count: {}",
        msg,
        storage.messages.count()
    );

    // todo fetch all endpoints for app
    // filter by topics
    // todo dispatch

    Ok(HttpResponse::Created())
}
