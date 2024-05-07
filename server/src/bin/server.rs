use std::net::TcpListener;

use dotenv::dotenv;
use envconfig::Envconfig;

use server::app::run;
use server::config::ServerConfig;
use server::logs::init_log;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    dotenv().ok();
    init_log();

    let config = ServerConfig::init_from_env().unwrap();
    let listener = TcpListener::bind((config.host, config.port))
        .unwrap_or_else(|_| panic!("Failed to bind port {}", config.port));

    run(listener).await?.await
}
