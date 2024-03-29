use crate::configuration::handlers::{create_application_handler, create_endpoint_handler};
use crate::events::handlers::create_message_handler;
use actix_web::web;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/v1")
            .route("/application", web::post().to(create_application_handler))
            .route(
                "/application/{app_id}/endpoint",
                web::post().to(create_endpoint_handler),
            )
            .route(
                "application/{app_id}/message",
                web::post().to(create_message_handler),
            ),
    );
}
