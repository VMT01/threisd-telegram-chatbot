mod user;

#[macro_export]
macro_rules! BaseEntity {
    (#[$tname:expr] $vis:vis struct $name:ident { $($fvis:vis $field:ident: $type:ty,)* }) => {
        #[derive(Debug, Clone, diesel::Queryable, diesel::Selectable, diesel::Insertable, serde::Serialize, serde::Deserialize)]
        #[diesel(table_name = $tname)]
        #[diesel(check_for_backend(diesel::sqlite::Sqlite))]
        $vis struct $name {
            pub id: Option<i32>,
            pub created_at: String,
            pub updated_at: String,
            $($fvis $field: $type,)*
        }

        impl $name {
            $vis fn new($($field:$type,)*) -> Self {
                let now: String = chrono::Utc::now().naive_utc().to_string();

                Self {
                    id: None,
                    created_at: now.clone(),
                    updated_at: now,
                    $($field,)*
                }
            }
        }
    }
}

pub use self::user::User;
