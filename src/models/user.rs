use diesel::prelude::*;

use crate::{configs::DATABASE_CONFIG, BaseEntity};

BaseEntity! {
    #[crate::schema::users]
    pub struct User {
        uid: String,
        uemail: String,
    }
}

impl User {
    pub fn find_one_by_user_id(uid: String) -> Option<User> {
        use crate::schema::users;
        match &mut DATABASE_CONFIG.pool.get() {
            Ok(conn) => {
                if let Ok(user) = users::table
                    .filter(users::uid.eq(uid))
                    .select(User::as_select())
                    .first(conn)
                {
                    return Some(user);
                }

                None
            }
            Err(err) => {
                log::error!("{}", err);
                None
            }
        }
    }

    pub fn create_user(uid: String, uemail: String) -> Result<(), ()> {
        use crate::schema::users;

        let new_user = User::new(uid, uemail);

        match &mut DATABASE_CONFIG.pool.get() {
            Ok(conn) => {
                if diesel::insert_into(users::table)
                    .values(&new_user)
                    .execute(conn)
                    .is_ok()
                {
                    return Ok(());
                }
                Err(())
            }
            Err(err) => {
                log::error!("{}", err);
                Err(())
            }
        }
    }
}
