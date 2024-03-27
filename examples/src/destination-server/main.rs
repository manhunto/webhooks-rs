use actix_web::rt::time::sleep;
use actix_web::web::Payload;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use futures::StreamExt;
use rand::Rng;
use std::time::Duration;
use web::BytesMut;

async fn index(payload: Payload) -> impl Responder {
    let mut rng = rand::thread_rng();
    let delay = rng.gen_range(40..=300);

    let body = get_body(payload).await;

    println!("Request. Delay: {} ms :: Body: {}", delay, body,);

    sleep(Duration::from_millis(delay)).await;

    HttpResponse::NoContent()
}

async fn get_body(mut payload: Payload) -> String {
    let mut bytes = BytesMut::new();
    while let Some(item) = payload.next().await {
        let item = item.unwrap();
        bytes.extend_from_slice(&item);
    }

    String::from_utf8_lossy(&bytes).to_string()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let ip = "127.0.0.1";
    let port = 8080;

    println!("Server is listening for requests on {}:{}", ip, port);

    HttpServer::new(|| App::new().route("/", web::post().to(index)))
        .bind((ip, port))?
        .run()
        .await
}
