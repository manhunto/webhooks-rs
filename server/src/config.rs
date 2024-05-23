use envconfig::Envconfig;

#[derive(Envconfig, Clone)]
pub struct ServerConfig {
    #[envconfig(from = "SERVER_PORT")]
    pub port: u16,
    #[envconfig(from = "SERVER_HOST")]
    pub host: String,
}

#[derive(Envconfig)]
pub struct PostgresConfig {
    #[envconfig(from = "POSTGRES_HOST")]
    host: String,
    #[envconfig(from = "POSTGRES_PORT")]
    port: u16,
    #[envconfig(from = "POSTGRES_USER")]
    user: String,
    #[envconfig(from = "POSTGRES_PASSWORD")]
    password: String,
    #[envconfig(from = "POSTGRES_DB")]
    db: String,
}

impl PostgresConfig {
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.user, self.password, self.host, self.port, self.db
        )
    }

    pub fn connection_string_without_db(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}",
            self.user, self.password, self.host, self.port
        )
    }

    pub fn with_db(&self, db: &str) -> Self {
        Self {
            host: self.host.clone(),
            port: self.port,
            user: self.user.clone(),
            password: self.password.clone(),
            db: db.to_string(),
        }
    }

    pub fn db(&self) -> String {
        self.db.clone()
    }
}

#[derive(Envconfig, Clone)]
pub struct AMQPConfig {
    #[envconfig(from = "AMQP_HOST")]
    host: String,
    #[envconfig(from = "AMQP_PORT")]
    port: u16,
    #[envconfig(from = "AMQP_USER")]
    user: String,
    #[envconfig(from = "AMQP_PASSWORD")]
    password: String,
    #[envconfig(from = "AMQP_SENT_MESSAGE_QUEUE")]
    sent_message_queue: String,
}

impl AMQPConfig {
    pub fn connection_string(&self) -> String {
        format!(
            "amqp://{}:{}@{}:{}",
            self.user, self.password, self.host, self.port
        )
    }

    pub fn with_sent_message_queue(&self, queue_name: &str) -> Self {
        Self {
            host: self.host.clone(),
            port: self.port,
            user: self.user.clone(),
            password: self.password.clone(),
            sent_message_queue: queue_name.to_string(),
        }
    }

    pub fn sent_message_queue_name(&self) -> String {
        self.sent_message_queue.clone()
    }

    pub fn sent_message_exchange_name(&self) -> String {
        format!("{}-exchange", self.sent_message_queue)
    }
}
