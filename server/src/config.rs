use envconfig::Envconfig;

#[derive(Envconfig)]
pub struct ServerConfig {
    #[envconfig(from = "SERVER_PORT")]
    pub port: u16,
    #[envconfig(from = "SERVER_HOST")]
    pub host: String,
}
