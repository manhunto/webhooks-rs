use envconfig::Envconfig;

#[derive(Envconfig)]
pub struct ServerConfig {
    #[envconfig(from = "SERVER_PORT")]
    pub port: u16,
}
