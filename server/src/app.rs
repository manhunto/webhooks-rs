use std::net::TcpListener;

use actix_web::dev::Server;
use actix_web::middleware::Logger;
use actix_web::web::Data;
use actix_web::{App, HttpServer};
use log::info;
use sqlx::PgPool;

use crate::amqp::{establish_connection_with_rabbit, Publisher};
use crate::config::AMQPConfig;
use crate::dispatch_consumer::consume;
use crate::routes::routes;
use crate::storage::Storage;

pub async fn run_server(
    listener: TcpListener,
    pool: PgPool,
    amqp_config: AMQPConfig,
) -> Result<Server, std::io::Error> {
    let channel = establish_connection_with_rabbit(amqp_config.clone()).await;
    let storage = Data::new(Storage::new(pool));
    let publisher = Data::new(Publisher::new(channel.clone(), amqp_config));
    let app = move || {
        App::new()
            .wrap(Logger::default())
            .app_data(storage.clone())
            .app_data(publisher.clone())
            .configure(routes)
    };

    let addr = listener.local_addr().unwrap();
    let server = HttpServer::new(app).listen(listener)?.run();

    info!("Webhooks server is listening for requests on {}", addr);

    Ok(server)
}

pub async fn run_dispatcher(pool: PgPool, amqp_config: AMQPConfig) {
    let channel = establish_connection_with_rabbit(amqp_config.clone()).await;

    consume(channel, "dispatcher", Storage::new(pool), amqp_config).await;
}
