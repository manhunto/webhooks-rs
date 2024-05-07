use dotenv::dotenv;
use envconfig::Envconfig;

use server::app::spawn_app;
use server::config::ServerConfig;
use server::logs::init_log;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    init_log();

    let config = ServerConfig::init_from_env().unwrap();

    spawn_app(config).await?.await
}
