pub mod config;
use ::config::ConfigError;
use once_cell::sync::Lazy;
use sea_orm::Database;
use sea_orm::DatabaseConnection;

// Create a static database connection
static DB_CONN: Lazy<DatabaseConnection> = Lazy::new(|| {
    async_std::task::block_on(async {
        Database::connect(format!("{}", CONFIG.database.url))
            .await
            .expect("Failed to connect to the database")
    })
});

pub static CONFIG: Lazy<config::AppConfig> = Lazy::new(|| get_config().unwrap());

// Accessor function for the connection
pub fn db_connection() -> &'static DatabaseConnection {
    &DB_CONN
}

pub fn get_config() -> Result<config::AppConfig, ConfigError> {
    config::AppConfig::from_env()
}
