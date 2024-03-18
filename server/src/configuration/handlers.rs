use crate::configuration::domain::{Application, ApplicationId, Endpoint, Topic};
use crate::configuration::models::{
    CreateAppRequest, CreateAppResponse, CreateEndpointRequest, CreateEndpointResponse,
};
use crate::storage::Storage;
use actix_web::web::{Data, Json, Path};
use actix_web::{HttpResponse, Responder};
use log::debug;
use serde_json::json;

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
    let app_id = ApplicationId::try_from(path.into_inner()).unwrap();

    if !storage.applications.exists(&app_id) {
        // todo unify errors
        return HttpResponse::NotFound().json(json!({"error": "application not found"}));
    }

    let url = request.url.clone();
    let topics = request
        .topics
        .clone()
        .into_iter()
        .map(|t| Topic::new(t).unwrap()) // todo handle error
        .collect();

    let endpoint = Endpoint::new(url, app_id, topics);

    storage.endpoints.save(endpoint.clone());

    debug!(
        "Endpoint created: {:?}, count: {}",
        endpoint,
        storage.applications.count()
    );

    let response = CreateEndpointResponse::from(endpoint);

    HttpResponse::Created().json(response)
}
