// @generated automatically by Diesel CLI.

diesel::table! {
    shortenedurl (id) {
        id -> Text,
        url -> Nullable<Text>,
        shorturl -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
