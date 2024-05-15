use std::net::TcpListener;

use server::app::run_without_rabbit_mq;

pub fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let addr = format!("http://{}", listener.local_addr().unwrap());
    let server = run_without_rabbit_mq(listener).unwrap();

    #[allow(clippy::let_underscore_future)]
    let _ = tokio::spawn(server);

    addr
}
