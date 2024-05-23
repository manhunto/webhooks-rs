use std::net::TcpListener;

use dotenv::dotenv;
use envconfig::Envconfig;
use sqlx::PgPool;

use server::app::run_server;
use server::config::{AMQPConfig, PostgresConfig, ServerConfig};
use server::logs::init_log;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    dotenv().ok();
    init_log();

    let config = ServerConfig::init_from_env().unwrap();
    let listener = TcpListener::bind((config.host, config.port))
        .unwrap_or_else(|_| panic!("Failed to bind port {}", config.port));

    let con_string = PostgresConfig::init_from_env().unwrap().connection_string();
    let pool = PgPool::connect(&con_string).await.unwrap();

    let amqp_config = AMQPConfig::init_from_env().unwrap();

    run_server(listener, pool, amqp_config).await?.await
}
