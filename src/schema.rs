// @generated automatically by Diesel CLI.

diesel::table! {
    note (id) {
        id -> Integer,
        name -> Text,
        content -> Text,
        profile_id -> Integer,
    }
}

diesel::table! {
    profile (id) {
        id -> Integer,
        name -> Text,
        active -> Bool,
    }
}

diesel::allow_tables_to_appear_in_same_query!(note, profile);
