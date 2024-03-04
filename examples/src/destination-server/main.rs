use actix_web::rt::time::sleep;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use rand::Rng;
use std::time::Duration;

async fn index() -> impl Responder {
    let mut rng = rand::thread_rng();
    let delay = rng.gen_range(40..=300);

    println!("Request with response delay {} ms", delay);

    sleep(Duration::from_millis(delay)).await;

    HttpResponse::NoContent()
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
