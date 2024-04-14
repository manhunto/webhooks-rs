use server::amqp::establish_connection_with_rabbit;
use server::dispatch_consumer::consume;
use server::logs::init_log;

#[tokio::main]
async fn main() {
    init_log();
    let channel = establish_connection_with_rabbit().await;

    consume(channel, "dispatcher").await
}
