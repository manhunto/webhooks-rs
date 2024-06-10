use std::net::TcpListener;

use dotenv::dotenv;
use envconfig::Envconfig;
use fake::{Fake, Faker};
use reqwest::Client;
use serde_json::{json, Value};
use sqlx::{migrate, Connection, Executor, PgConnection, PgPool};
use svix_ksuid::{Ksuid, KsuidLike};

use server::app::{run_dispatcher, run_server};
use server::config::{AMQPConfig, PostgresConfig};
use server::logs::init_log;
use server::storage::Storage;
use server::types::{ApplicationId, EndpointId};

struct TestEnvironmentBuilder;

impl TestEnvironmentBuilder {
    pub async fn build() -> TestEnvironment {
        dotenv().ok();

        let test_id = Ksuid::new(None, None).to_base62();

        TestEnvironment {
            pool: Self::prepare_db(test_id.as_str()).await,
            amqp_config: Self::prepare_amqp(test_id.as_str()),
        }
    }

    pub async fn build_with_logs() -> TestEnvironment {
        init_log();

        Self::build().await
    }

    async fn prepare_db(test_id: &str) -> PgPool {
        // Create db
        let pg_config = PostgresConfig::init_from_env().unwrap();
        let mut connection = PgConnection::connect(&pg_config.connection_string_without_db())
            .await
            .expect("Failed to connect to postgres");

        let pg_config = pg_config.with_db(test_id);

        connection
            .execute(format!(r#"CREATE DATABASE "{}";"#, pg_config.db()).as_str())
            .await
            .expect("Failed to create database.");

        // Migrate db
        let pool = PgPool::connect(&pg_config.connection_string())
            .await
            .expect("Failed to connect to postgres");

        migrate!("./migrations")
            .run(&pool)
            .await
            .expect("Failed to migrate");

        pool
    }

    fn prepare_amqp(test_id: &str) -> AMQPConfig {
        let config = AMQPConfig::init_from_env().unwrap();

        config.with_sent_message_queue(test_id)
    }
}

pub struct TestEnvironment {
    pool: PgPool,
    amqp_config: AMQPConfig,
}

impl TestEnvironment {
    pub async fn new() -> Self {
        TestEnvironmentBuilder::build().await
    }

    #[allow(dead_code)]
    pub async fn new_with_logs() -> Self {
        TestEnvironmentBuilder::build_with_logs().await
    }

    pub async fn server(&self) -> TestServer {
        TestServerBuilder::new(self.pool.clone(), self.amqp_config.clone())
            .run()
            .await
    }

    pub async fn dispatcher(&self) {
        TestDispatcherBuilder::new(self.pool.clone(), self.amqp_config.clone())
            .run()
            .await
    }
}

struct TestServerBuilder {
    pool: PgPool,
    amqp_config: AMQPConfig,
}

impl TestServerBuilder {
    fn new(pool: PgPool, amqp_config: AMQPConfig) -> Self {
        Self { pool, amqp_config }
    }

    async fn run(&self) -> TestServer {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = format!("http://{}", listener.local_addr().unwrap());

        let server = run_server(listener, self.pool.clone(), self.amqp_config.clone())
            .await
            .unwrap();

        #[allow(clippy::let_underscore_future)]
        let _ = tokio::spawn(server);

        TestServer {
            server_url: addr,
            storage: Storage::new(self.pool.clone()),
        }
    }
}

pub struct TestServer {
    server_url: String,
    storage: Storage,
}

impl TestServer {
    pub fn url(&self, endpoint: &str) -> String {
        format!("{}/{}", self.base_url(), endpoint)
    }

    fn base_url(&self) -> String {
        self.server_url.to_string()
    }

    pub fn storage(&self) -> &Storage {
        &self.storage
    }
}

struct TestDispatcherBuilder {
    pool: PgPool,
    amqp_config: AMQPConfig,
}

impl TestDispatcherBuilder {
    fn new(pool: PgPool, amqp_config: AMQPConfig) -> Self {
        Self { pool, amqp_config }
    }

    async fn run(&self) {
        let pool = self.pool.clone();
        let amqp_config = self.amqp_config.clone();

        #[allow(clippy::let_underscore_future)]
        tokio::spawn(async move { run_dispatcher(pool, amqp_config).await });
    }
}

macro_rules! run_test_server {
    () => {
        TestEnvironment::new().await.server().await
    };
}

macro_rules! run_test_server_and_dispatcher {
    () => {{
        let environment = TestEnvironment::new().await;
        let server = environment.server().await;

        environment.dispatcher().await;

        server
    }};
}

macro_rules! assert_mock_with_retry {
    ($mock: ident) => {{
        let mut attempt: u8 = 1;
        const MAX_ATTEMPTS: u8 = 10;
        let mock: mockito::Mock = $mock;

        while !mock.matched_async().await && attempt <= MAX_ATTEMPTS {
            let delay = std::time::Duration::from_millis((10 * attempt).into());

            println!("Delay for asserting mockito mock server: {:?}", delay);

            tokio::time::sleep(delay).await;
            attempt += 1;
        }

        mock.assert_async().await;
    }};
}

pub(crate) use assert_mock_with_retry;
pub(crate) use run_test_server;
pub(crate) use run_test_server_and_dispatcher;

pub struct Given {
    url: String,
}

impl Given {
    fn new(url: String) -> Given {
        Self { url }
    }

    pub async fn app(&self) -> ApplicationId {
        let name: String = Faker.fake::<String>();

        let response = Client::new()
            .post(&format!("{}/application", self.url))
            .json(&json!({
              "name": name
            }))
            .send()
            .await
            .expect("Failed to executed request");

        let body = response.json::<Value>().await.unwrap();

        let id = ApplicationId::try_from(body["id"].as_str().unwrap().to_string())
            .expect("Invalid application id");

        id
    }

    pub async fn endpoint_with_app(
        &self,
        url: &str,
        topics: Vec<&str>,
    ) -> (ApplicationId, EndpointId) {
        let app_id = self.app().await;

        let response = Client::new()
            .post(&format!("{}/application/{}/endpoint", self.url, app_id))
            .json(&json!({
              "url": url,
              "topics": topics
            }))
            .send()
            .await
            .expect("Failed to executed request");

        let body = response.json::<Value>().await.unwrap();

        let endpoint_id = EndpointId::try_from(body["id"].as_str().unwrap().to_string())
            .expect("Invalid endpoint id");

        (app_id, endpoint_id)
    }

    pub async fn disable_endpoint(&self, app_id: &ApplicationId, endpoint_id: &EndpointId) {
        Client::new()
            .post(&format!(
                "{}/application/{}/endpoint/{}/disable",
                self.url, app_id, endpoint_id
            ))
            .send()
            .await
            .expect("Failed to executed request");
    }
}

impl From<&TestServer> for Given {
    fn from(value: &TestServer) -> Self {
        Self::new(value.base_url())
    }
}
