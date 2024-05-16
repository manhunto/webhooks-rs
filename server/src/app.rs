use std::net::TcpListener;

use actix_web::dev::Server;
use actix_web::web::Data;
use actix_web::{rt, App, HttpServer};
use envconfig::Envconfig;
use log::info;
use sqlx::PgPool;

use crate::amqp::{establish_connection_with_rabbit, Publisher};
use crate::config::PostgresConfig;
use crate::dispatch_consumer::consume;
use crate::routes::routes;
use crate::storage::Storage;

pub async fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let con_string = PostgresConfig::init_from_env().unwrap().connection_string();
    let pool = PgPool::connect(&con_string).await.unwrap();

    let channel = establish_connection_with_rabbit().await;
    let storage = Data::new(Storage::new(pool));
    let storage_for_consumer = storage.clone();
    let publisher = Data::new(Publisher::new(channel.clone()));
    let app = move || {
        App::new()
            .app_data(storage.clone())
            .app_data(publisher.clone())
            .configure(routes)
    };

    rt::spawn(async move { consume(channel, "dispatcher-in-server", storage_for_consumer).await });

    let addr = listener.local_addr().unwrap();
    let server = HttpServer::new(app).listen(listener)?.run();

    info!("Webhooks server is listening for requests on {}", addr);

    Ok(server)
}

// fixme: extract one build function
pub async fn run_without_rabbit_mq(
    listener: TcpListener,
    pool: PgPool,
) -> Result<Server, std::io::Error> {
    let storage = Data::new(Storage::new(pool));

    let app = move || {
        App::new()
            .app_data(Data::new(storage.clone()))
            .configure(routes)
    };

    let addr = listener.local_addr().unwrap();
    let server = HttpServer::new(app).listen(listener)?.run();

    info!("Webhooks server is listening for requests on {}", addr);

    Ok(server)
}
