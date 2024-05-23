use std::net::TcpListener;

use dotenv::dotenv;
use envconfig::Envconfig;
use fake::{Fake, Faker};
use reqwest::Client;
use serde_json::{json, Value};
use sqlx::{migrate, Connection, Executor, PgConnection, PgPool};
use svix_ksuid::{Ksuid, KsuidLike};

use server::app::{run_dispatcher, run_server};
use server::config::PostgresConfig;
use server::logs::init_log;
use server::storage::Storage;
use server::types::{ApplicationId, EndpointId};

struct TestEnvironmentBuilder;

impl TestEnvironmentBuilder {
    pub async fn build() -> TestEnvironment {
        dotenv().ok();

        TestEnvironment {
            pool: Self::prepare_db().await,
        }
    }

    pub async fn build_with_logs() -> TestEnvironment {
        dotenv().ok();
        init_log();

        TestEnvironment {
            pool: Self::prepare_db().await,
        }
    }

    async fn prepare_db() -> PgPool {
        // Create db
        let pg_config = PostgresConfig::init_from_env().unwrap();
        let mut connection = PgConnection::connect(&pg_config.connection_string_without_db())
            .await
            .expect("Failed to connect to postgres");

        let random_db_name = Ksuid::new(None, None).to_base62();
        let pg_config = pg_config.with_db(random_db_name.as_str());

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
}

pub struct TestEnvironment {
    pool: PgPool,
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
        TestServerBuilder::new(self.pool.clone()).run().await
    }

    #[allow(dead_code)]
    pub async fn dispatcher(&self) {
        TestDispatcherBuilder::new(self.pool.clone()).run().await
    }
}

struct TestServerBuilder {
    pool: PgPool,
}

impl TestServerBuilder {
    fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    async fn run(&self) -> TestServer {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = format!("http://{}", listener.local_addr().unwrap());

        let server = run_server(listener, self.pool.clone()).await.unwrap();

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
        format!("{}/v1", self.server_url)
    }

    #[allow(dead_code)]
    pub fn storage(&self) -> &Storage {
        &self.storage
    }
}

struct TestDispatcherBuilder {
    pool: PgPool,
}

impl TestDispatcherBuilder {
    fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    async fn run(&self) {
        let pool = self.pool.clone();

        #[allow(clippy::let_underscore_future)]
        tokio::spawn(async move { run_dispatcher(pool).await });
    }
}

#[allow(unused_macros)]
macro_rules! run_test_server {
    () => {
        TestEnvironment::new().await.server().await
    };
}

#[allow(unused_macros)]
macro_rules! run_test_server_and_dispatcher {
    () => {{
        let environment = TestEnvironment::new().await;
        let server = environment.server().await;

        environment.dispatcher().await;

        server
    }};
}

#[allow(unused_imports)]
pub(crate) use run_test_server;
#[allow(unused_imports)]
pub(crate) use run_test_server_and_dispatcher;

#[allow(dead_code)]
pub struct Given {
    url: String,
}

#[allow(dead_code)]
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
}

impl From<&TestServer> for Given {
    fn from(value: &TestServer) -> Self {
        Self::new(value.base_url())
    }
}
