use crate::application::handlers::create_handler;
use actix_web::web;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/v1").route("/application", web::post().to(create_handler)));
}
