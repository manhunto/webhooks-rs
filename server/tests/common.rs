use std::net::TcpListener;

use dotenv::dotenv;
use envconfig::Envconfig;
use sqlx::{migrate, Connection, Executor, PgConnection, PgPool, Pool, Postgres};
use svix_ksuid::{Ksuid, KsuidLike};

use server::app::run_without_rabbit_mq;
use server::config::PostgresConfig;

struct TestServerBuilder;

impl TestServerBuilder {
    async fn run() -> TestServer {
        dotenv().ok();

        let pool = Self::prepare_db().await;

        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = format!("http://{}", listener.local_addr().unwrap());

        let server = run_without_rabbit_mq(listener, pool).await.unwrap();

        #[allow(clippy::let_underscore_future)]
        let _ = tokio::spawn(server);

        TestServer { server_url: addr }
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
}

impl TestServer {
    pub async fn run() -> Self {
        TestServerBuilder::run().await
    }

    pub fn url(&self, endpoint: &str) -> String {
        format!("{}/v1/{}", self.server_url, endpoint)
    }
}
