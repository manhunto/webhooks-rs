use actix_web::web::Data;
use actix_web::{App, HttpServer};
use log::info;
use server::amqp::establish_connection_with_rabbit;
use server::logs::init_log;
use server::routes::routes;
use server::storage::Storage;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    init_log();

    let ip = "127.0.0.1";
    let port = 8090;

    let storage = Data::new(Storage::new());
    let rabbit_channel = Data::new(establish_connection_with_rabbit().await);
    let app = move || {
        App::new()
            .app_data(storage.clone())
            .app_data(rabbit_channel.clone())
            .configure(routes)
    };

    info!(
        "Webhooks server is listening for requests on {}:{}",
        ip, port
    );

    HttpServer::new(app).bind((ip, port))?.run().await
}
