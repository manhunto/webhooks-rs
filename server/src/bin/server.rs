use actix_web::web::Data;
use actix_web::{App, HttpServer};
use lapin::options::QueueDeclareOptions;
use lapin::types::FieldTable;
use lapin::{Channel, Connection, ConnectionProperties};
use log::info;
use server::logs::init_log;
use server::routes::routes;
use server::storage::Storage;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    init_log();

    let ip = "127.0.0.1";
    let port = 8090;

    let storage = Data::new(Storage::new());
    let rabbit_channel = Data::new(establish_connection_with_rabbit().await);
    let app = move || {
        App::new()
            .app_data(storage.clone())
            .app_data(rabbit_channel.clone())
            .configure(routes)
    };

    info!(
        "Webhooks server is listening for requests on {}:{}",
        ip, port
    );

    HttpServer::new(app).bind((ip, port))?.run().await
}

async fn establish_connection_with_rabbit() -> Channel {
    let addr = "amqp://guest:guest@localhost:5672";
    let conn = Connection::connect(addr, ConnectionProperties::default())
        .await
        .unwrap();

    info!("connected established with rabbitmq");

    let channel = conn.create_channel().await.unwrap();

    let queue = channel
        .queue_declare(
            "sent_message",
            QueueDeclareOptions::default(),
            FieldTable::default(),
        )
        .await
        .unwrap();

    info!("queue declared {:?}", queue);

    channel
}
