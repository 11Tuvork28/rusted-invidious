use envconfig::Envconfig;

#[derive(Envconfig)]
pub struct Config {
    #[envconfig(from = "DB_HOST")]
    pub db_host: String,
    #[envconfig(from = "DB_PORT", default = "5432")]
    pub db_port: u16,
    #[envconfig(from = "LOG_LEVEL", default = "debug")]
    pub log_level: String,
}