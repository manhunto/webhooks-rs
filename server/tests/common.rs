use std::net::TcpListener;

use dotenv::dotenv;
use envconfig::Envconfig;
use fake::{Fake, Faker};
use reqwest::Client;
use serde_json::{json, Value};
use sqlx::{migrate, Connection, Executor, PgConnection, PgPool, Pool, Postgres};
use svix_ksuid::{Ksuid, KsuidLike};

use server::app::run_without_rabbit_mq;
use server::config::PostgresConfig;
use server::logs::init_log;
use server::storage::Storage;
use server::types::ApplicationId;

#[derive(Default)]
struct TestServerBuilder {
    logs: bool,
}

impl TestServerBuilder {
    fn with_logs(&mut self) -> &mut Self {
        self.logs = true;
        self
    }

    async fn run(&self) -> TestServer {
        dotenv().ok();

        if cfg!(not(tarpaulin_include)) && self.logs {
            init_log();
        }

        let pool = Self::prepare_db().await;

        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = format!("http://{}", listener.local_addr().unwrap());

        let server = run_without_rabbit_mq(listener, pool.clone()).await.unwrap();

        #[allow(clippy::let_underscore_future)]
        let _ = tokio::spawn(server);

        TestServer {
            server_url: addr,
            storage: Storage::new(pool),
        }
    }

    async fn prepare_db() -> Pool<Postgres> {
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

pub struct TestServer {
    server_url: String,
    storage: Storage,
}

impl TestServer {
    pub async fn run() -> Self {
        TestServerBuilder::default().run().await
    }

    #[allow(dead_code)]
    #[cfg(not(tarpaulin_include))]
    pub async fn run_with_logs() -> Self {
        TestServerBuilder::default().with_logs().run().await
    }

    pub fn url(&self, endpoint: &str) -> String {
        format!("{}/{}", self.base_url(), endpoint)
    }

    pub fn base_url(&self) -> String {
        format!("{}/v1", self.server_url)
    }

    #[allow(dead_code)]
    pub fn storage(&self) -> &Storage {
        &self.storage
    }
}

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
}

impl From<&TestServer> for Given {
    fn from(value: &TestServer) -> Self {
        Self::new(value.base_url())
    }
}
