use crate::configuration::domain::{Application, ApplicationId, Endpoint};
use crate::configuration::models::{
    CreateAppRequest, CreateAppResponse, CreateEndpointRequest, CreateEndpointResponse,
};
use crate::storage::Storage;
use actix_web::web::{Data, Json, Path};
use actix_web::{HttpResponse, Responder};
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
