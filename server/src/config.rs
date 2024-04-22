use envconfig::Envconfig;

#[derive(Envconfig)]
pub struct ServerConfig {
    #[envconfig(from = "SERVER_PORTS")]
    pub port: u16,
}
