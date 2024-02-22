// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Integer,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        uid -> Text,
    }
}
