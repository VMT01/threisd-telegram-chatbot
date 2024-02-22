use diesel::{
    r2d2::{ConnectionManager, Pool},
    sqlite::SqliteConnection,
};

use crate::{constants::ENV_CONSTANTS, types::SqlitePool};

pub struct DatabaseConfig {
    pub sqlite_pool: SqlitePool,
}

impl DatabaseConfig {
    pub fn init() -> Self {
        let sqlite_pool = Pool::builder()
            .build(ConnectionManager::<SqliteConnection>::new(
                &ENV_CONSTANTS.database_url,
            ))
            .unwrap_or_else(|e| panic!("Failed to establish connection: {}", e));

        Self { sqlite_pool }
    }
}
