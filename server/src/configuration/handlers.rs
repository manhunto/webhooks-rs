use actix_web::web::{Data, Json, Path};
use actix_web::{HttpResponse, Responder};
use itertools::Itertools;
use log::debug;

use crate::configuration::domain::{Application, ApplicationId, Endpoint, EndpointId, Topic};
use crate::configuration::models::{
    CreateAppRequest, CreateAppResponse, CreateEndpointRequest, CreateEndpointResponse,
};
use crate::error::ResponseError;
use crate::storage::Storage;

pub async fn create_application_handler(
    storage: Data<Storage>,
    request: Json<CreateAppRequest>,
) -> impl Responder {
    let app = Application::new(request.name.to_string());

    storage.applications.save(app.clone());

    debug!(
        "Application created: {:?}, count: {}",
        app,
        storage.applications.count()
    );

    let response = CreateAppResponse::from(app);

    HttpResponse::Created().json(response)
}

pub async fn create_endpoint_handler(
    storage: Data<Storage>,
    request: Json<CreateEndpointRequest>,
    path: Path<String>,
) -> Result<impl Responder, ResponseError> {
    let app_id = ApplicationId::try_from(path.into_inner()).unwrap();
    let app = storage.applications.get(&app_id)?;

    let url = request.url.clone();
    let topics = request
        .topics
        .clone()
        .into_iter()
        .map(Topic::new)
        .try_collect()?;

    let endpoint = Endpoint::new(url, app.id, topics);

    storage.endpoints.save(endpoint.clone());

    debug!(
        "Endpoint created: {:?}, count: {}",
        endpoint,
        storage.endpoints.count()
    );

    let response = CreateEndpointResponse::from(endpoint);

    Ok(HttpResponse::Created().json(response))
}

pub async fn disable_endpoint_handler(
    storage: Data<Storage>,
    path: Path<(String, String)>,
) -> Result<impl Responder, ResponseError> {
    handle_status(storage, path, StatusAction::Disable).await
}

pub async fn enable_endpoint_handler(
    storage: Data<Storage>,
    path: Path<(String, String)>,
) -> Result<impl Responder, ResponseError> {
    handle_status(storage, path, StatusAction::Enable).await
}

enum StatusAction {
    Enable,
    Disable,
}

async fn handle_status(
    storage: Data<Storage>,
    path: Path<(String, String)>,
    action: StatusAction,
) -> Result<impl Responder, ResponseError> {
    let (app_id, endpoint_id) = path.into_inner();

    let app_id = ApplicationId::try_from(app_id).unwrap();
    let app = storage.applications.get(&app_id)?;

    let endpoint_id = EndpointId::try_from(endpoint_id).unwrap();
    let mut endpoint = storage.endpoints.get(&endpoint_id)?;

    if !endpoint.app_id.eq(&app.id) {
        // todo get endpoint with one query - app_id + endpoint_id
        return Err(ResponseError::NotFound("Endpoint not found".to_string()));
    }

    match action {
        StatusAction::Enable => endpoint.enable_manually(),
        StatusAction::Disable => endpoint.disable_manually(),
    }

    storage.endpoints.save(endpoint);

    match action {
        StatusAction::Enable => debug!("Endpoint {} enabled", endpoint_id),
        StatusAction::Disable => debug!("Endpoint {} disabled", endpoint_id),
    }

    Ok(HttpResponse::NoContent())
}
