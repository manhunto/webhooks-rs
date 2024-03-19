use actix_web::web::Data;
use actix_web::{App, HttpServer};
use log::debug;
use server::logs::init_log;
use server::routes::routes;
use server::storage::Storage;

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
