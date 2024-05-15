use actix_web::web::Data;
use dotenv::dotenv;
use envconfig::Envconfig;
use sqlx::PgPool;

use server::amqp::establish_connection_with_rabbit;
use server::config::PostgresConfig;
use server::dispatch_consumer::consume;
use server::logs::init_log;
use server::storage::Storage;

#[tokio::main]
async fn main() {
    dotenv().ok();
    init_log();

    let con_string = PostgresConfig::init_from_env().unwrap().connection_string();
    let pool = PgPool::connect(&con_string).await.unwrap();

    let channel = establish_connection_with_rabbit().await;

    consume(channel, "dispatcher", Data::new(Storage::new(pool))).await
}
