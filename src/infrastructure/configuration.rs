use config::{Config, ConfigError, Environment};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub server_host: String,
    pub server_port: u32,

    pub db_host: String,
    pub db_port: u32,
    pub db_user: String,
    pub db_password: String,
    pub db_name: String,
}

impl AppConfig {
    pub fn db_uri(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}?sslmode=disable",
            self.db_user, self.db_password, self.db_host, self.db_port, self.db_name
        )
    }

    pub fn server_addr(&self) -> String {
        format!("{}:{}", self.server_host, self.server_port)
    }
}

// load from .env variable, but if .env not exist return using system environment
pub fn load() -> Result<AppConfig, ConfigError> {
    let _ = dotenvy::dotenv();

    let env = Environment::default().separator(".");

    let config = Config::builder().add_source(env).build();

    config.unwrap().try_deserialize()
}
