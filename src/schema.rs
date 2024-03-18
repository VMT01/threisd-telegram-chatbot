// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Nullable<Integer>,
        created_at -> Text,
        updated_at -> Text,
        uid -> Text,
        uemail -> Text,
    }
}
