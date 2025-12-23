// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Uuid,
        email -> Text,
        username -> Text,
        password_hash -> Text,
        created_at -> Timestamp,
    }
}
