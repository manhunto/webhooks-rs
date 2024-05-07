use actix_web::dev::Server;
use actix_web::web::Data;
use actix_web::{rt, App, HttpServer};
use log::info;

use crate::amqp::{establish_connection_with_rabbit, Publisher};
use crate::config::ServerConfig;
use crate::dispatch_consumer::consume;
use crate::routes::routes;
use crate::storage::Storage;

pub async fn spawn_app(config: ServerConfig) -> Result<Server, std::io::Error> {
    let storage = Data::new(Storage::new());
    let storage_for_consumer = storage.clone();
    let channel = establish_connection_with_rabbit().await;
    let publisher = Data::new(Publisher::new(channel.clone()));
    let app = move || {
        App::new()
            .app_data(storage.clone())
            .app_data(publisher.clone())
            .configure(routes)
    };

    rt::spawn(async move { consume(channel, "dispatcher-in-server", storage_for_consumer).await });

    let ip = config.host;
    let port: u16 = config.port;
    let server = HttpServer::new(app).bind((ip.clone(), port))?.run();

    info!(
        "Webhooks server is listening for requests on {}:{}",
        ip, port
    );

    Ok(server)
}
