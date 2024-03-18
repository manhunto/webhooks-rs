use crate::configuration::domain::{Application, ApplicationId, Endpoint, Topic};
use crate::configuration::models::{
    CreateAppRequest, CreateAppResponse, CreateEndpointRequest, CreateEndpointResponse,
};
use crate::error::ResponseError;
use crate::storage::Storage;
use actix_web::web::{Data, Json, Path};
use actix_web::{HttpResponse, Responder};
use itertools::Itertools;
use log::debug;

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
        storage.applications.count()
    );

    let response = CreateEndpointResponse::from(endpoint);

    Ok(HttpResponse::Created().json(response))
}
