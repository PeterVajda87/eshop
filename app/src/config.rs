use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Database {
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct Server {
    pub address: String,
    pub port: u16,
}

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub database: Database,
    pub server: Server,
}

impl AppConfig {
    pub fn from_env() -> Result<Self, ConfigError> {
        let env = std::env::var("APP_ENV").unwrap_or_else(|_| "devel".to_string());

        let builder = Config::builder()
            // Optional default configuration
            .add_source(File::with_name("src/config/default").required(false))
            // Environment-specific configuration
            .add_source(File::with_name(&format!("src/config/{}", env)).required(true))
            // Optional environment variable overrides
            .add_source(Environment::with_prefix("APP").separator("_"));

        let config = builder.build()?;
        config.try_deserialize()
    }
}
