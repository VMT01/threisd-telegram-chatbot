use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use log::error;

use crate::{configs::DATABASE_CONFIG, types::SqlitePooledConnection};

#[derive(Debug, Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct User {
    #[diesel(deserialize_as = i32)]
    pub id: Option<i32>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub uid: String,
}

impl Default for User {
    fn default() -> Self {
        let now = Utc::now().naive_utc();
        Self {
            id: None,
            created_at: now,
            updated_at: now,
            uid: "".to_string(),
        }
    }
}

impl User {
    pub fn find_user(user_id: String) -> Option<User> {
        use crate::schema::users;

        let connection: &mut SqlitePooledConnection =
            &mut DATABASE_CONFIG.sqlite_pool.get().unwrap();

        match users::table
            .filter(users::uid.eq(user_id))
            .select(User::as_select())
            .first(connection)
        {
            Ok(user) => Some(user),
            Err(err) => {
                error!("{}", err);
                None
            }
        }
    }

    pub fn create_user(user_id: String) -> Option<usize> {
        use crate::schema::users;

        let new_user = User {
            uid: user_id,
            ..Default::default()
        };

        let connection: &mut SqlitePooledConnection =
            &mut DATABASE_CONFIG.sqlite_pool.get().unwrap();

        match diesel::insert_into(users::table)
            .values(&new_user)
            .execute(connection)
        {
            Ok(res) => Some(res),
            Err(err) => {
                error!("{err}");
                None
            }
        }
    }
}
