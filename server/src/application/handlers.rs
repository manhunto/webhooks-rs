use crate::application::domain::Application;
use crate::application::models::{CreateAppRequest, CreateAppResponse};
use crate::storage::Storage;
use actix_web::web::{Data, Json};
use actix_web::{HttpResponse, Responder};
use log::debug;

pub async fn create_handler(
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
