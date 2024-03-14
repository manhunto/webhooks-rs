use crate::application::domain::ApplicationId;
use crate::endpoint::domain::Endpoint;
use crate::endpoint::models::{CreateEndpointRequest, CreateEndpointResponse};
use crate::storage::Storage;
use actix_web::web::{Data, Json, Path};
use actix_web::{HttpResponse, Responder};
use log::debug;

pub async fn create_endpoint_handler(
    storage: Data<Storage>,
    request: Json<CreateEndpointRequest>,
    path: Path<String>,
) -> impl Responder {
    let app_id = path.into_inner();
    // todo check if app exists

    let endpoint = Endpoint::new(
        request.url.clone(),
        ApplicationId::try_from(app_id).unwrap(),
    );

    storage.endpoints.save(endpoint.clone());

    debug!(
        "Endpoint created: {:?}, count: {}",
        endpoint,
        storage.applications.count()
    );

    let response = CreateEndpointResponse::from(endpoint);

    HttpResponse::Created().json(response)
}
