use lazy_static::lazy_static;

use self::database_config::DatabaseConfig;

mod database_config;

lazy_static! {
    pub static ref DATABASE_CONFIG: DatabaseConfig = DatabaseConfig::init();
}
