use actix_web::web;

use crate::configuration::handlers::{
    create_application_handler, create_endpoint_handler, disable_endpoint_handler,
    enable_endpoint_handler,
};
use crate::events::handlers::create_event_handler;
use crate::handlers::health_check::health_check;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/v1")
            .route("/health_check", web::get().to(health_check))
            .route("/application", web::post().to(create_application_handler))
            .route(
                "/application/{app_id}/endpoint",
                web::post().to(create_endpoint_handler),
            )
            .route(
                "/application/{app_id}/endpoint/{endpoint_id}/disable",
                web::post().to(disable_endpoint_handler),
            )
            .route(
                "/application/{app_id}/endpoint/{endpoint_id}/enable",
                web::post().to(enable_endpoint_handler),
            )
            .route(
                "application/{app_id}/event",
                web::post().to(create_event_handler),
            ),
    );
}
