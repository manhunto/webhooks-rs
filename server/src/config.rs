use envconfig::Envconfig;

#[derive(Envconfig, Clone)]
pub struct ServerConfig {
    #[envconfig(from = "SERVER_PORT")]
    pub port: u16,
    #[envconfig(from = "SERVER_HOST")]
    pub host: String,
}

impl ServerConfig {
    pub fn http_addr(&self) -> String {
        format!("http://{}:{}", self.host, self.port)
    }
}
