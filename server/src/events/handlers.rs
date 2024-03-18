use crate::configuration::domain::{ApplicationId, Topic};
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

    let endpoints = storage.endpoints.for_topic(&app_id, &topic);
    debug!("{} endpoints found for message {}", endpoints.len(), msg.id);

    for endpoint in endpoints {
        debug!("{} sending to {}", msg.id, endpoint.url);

        let response = reqwest::Client::new()
            .post(endpoint.url)
            .json(msg.payload.to_string().as_str())
            .send()
            .await;

        let dbg_msg = match response {
            Ok(res) => format!("Success! {}", res.status()),
            Err(res) => {
                let status: String = res.status().map_or(String::from("-"), |s| s.to_string());

                format!("Error response! Status: {}, Error: {}", status, res)
            }
        };

        debug!("{}", dbg_msg);
    }

    Ok(HttpResponse::Created())
}
