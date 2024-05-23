use dotenv::dotenv;
use envconfig::Envconfig;
use sqlx::PgPool;

use server::app::run_dispatcher;
use server::config::PostgresConfig;
use server::logs::init_log;

#[tokio::main]
async fn main() {
    dotenv().ok();
    init_log();

    let con_string = PostgresConfig::init_from_env().unwrap().connection_string();
    let pool = PgPool::connect(&con_string).await.unwrap();

    run_dispatcher(pool).await
}
