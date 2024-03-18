use diesel::{
    r2d2::{ConnectionManager, Pool},
    SqliteConnection,
};

use crate::{constants::ENV_CONSTANTS, types::SqlitePool};

pub struct DatabaseConfig {
    pub pool: SqlitePool,
}

impl DatabaseConfig {
    pub fn init() -> Self {
        let pool = Pool::builder()
            .build(ConnectionManager::<SqliteConnection>::new(
                &ENV_CONSTANTS.database_url,
            ))
            .unwrap_or_else(|err| panic!("Failed to establish connection: {}", err));

        Self { pool }
    }
}
