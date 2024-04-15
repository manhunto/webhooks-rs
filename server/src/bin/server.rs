use actix_web::web::Data;
use actix_web::{rt, App, HttpServer};
use dotenv::dotenv;
use log::info;

use server::amqp::{establish_connection_with_rabbit, Publisher};
use server::dispatch_consumer::consume;
use server::env::{Env, EnvVar};
use server::logs::init_log;
use server::routes::routes;
use server::storage::Storage;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    init_log();

    let ip = "127.0.0.1";
    let port: u16 = Env::env_or("SERVER_PORT", 8090);

    let storage = Data::new(Storage::new());
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

    rt::spawn(async move { consume(channel, "dispatcher-in-server").await });

    HttpServer::new(app).bind((ip, port))?.run().await
}
