use actix_web::web::Data;
use dotenv::dotenv;

use server::amqp::establish_connection_with_rabbit;
use server::dispatch_consumer::consume;
use server::logs::init_log;
use server::storage::Storage;

#[tokio::main]
async fn main() {
    dotenv().ok();
    init_log();
    let channel = establish_connection_with_rabbit().await;

    consume(channel, "dispatcher", Data::new(Storage::new())).await
}
