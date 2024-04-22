use actix_web::web::Data;
use actix_web::{rt, App, HttpServer};
use dotenv::dotenv;
use envconfig::Envconfig;
use log::info;

use server::amqp::{establish_connection_with_rabbit, Publisher};
use server::config::ServerConfig;
use server::dispatch_consumer::consume;
use server::logs::init_log;
use server::routes::routes;
use server::storage::Storage;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    init_log();

    let config = ServerConfig::init_from_env().unwrap();

    let ip = "127.0.0.1";
    let port: u16 = config.port;

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

    info!(
        "Webhooks server is listening for requests on {}:{}",
        ip, port
    );

    rt::spawn(async move { consume(channel, "dispatcher-in-server", storage_for_consumer).await });

    HttpServer::new(app).bind((ip, port))?.run().await
}
