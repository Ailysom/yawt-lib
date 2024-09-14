// @generated automatically by Diesel CLI.

diesel::table! {
    task (id) {
        id -> Text,
        description -> Text,
        priority -> Integer,
        published -> Bool,
        deadline -> Text,
        time_stamp -> Text,
    }
}
