mod configuration;
mod error;
mod logs;
mod routes;
mod storage;

use crate::logs::init_log;
use crate::routes::routes;
use crate::storage::Storage;
use actix_web::web::Data;
use actix_web::{App, HttpServer};
use log::debug;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    init_log();

    let ip = "127.0.0.1";
    let port = 8090;

    debug!(
        "Webhooks server is listening for requests on {}:{}",
        ip, port
    );

    let storage = Data::new(Storage::new());
    let app = move || App::new().app_data(storage.clone()).configure(routes);

    HttpServer::new(app).bind((ip, port))?.run().await
}
