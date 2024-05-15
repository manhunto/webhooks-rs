use std::net::TcpListener;

use dotenv::dotenv;

use server::app::run_without_rabbit_mq;

struct TestServerBuilder;

impl TestServerBuilder {
    async fn run() -> TestServer {
        dotenv().ok();

        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = format!("http://{}", listener.local_addr().unwrap());
        let server = run_without_rabbit_mq(listener).await.unwrap();

        #[allow(clippy::let_underscore_future)]
        let _ = tokio::spawn(server);

        TestServer { server_url: addr }
    }
}

pub struct TestServer {
    server_url: String,
}

impl TestServer {
    pub async fn run() -> Self {
        TestServerBuilder::run().await
    }

    pub fn url(&self, endpoint: &str) -> String {
        format!("{}/v1/{}", self.server_url, endpoint)
    }
}
