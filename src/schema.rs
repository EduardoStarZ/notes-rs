// @generated automatically by Diesel CLI.

diesel::table! {
    note (id) {
        id -> Integer,
        name -> Text,
        content -> Text,
        user_id -> Integer,
    }
}

diesel::table! {
    user (id) {
        id -> Integer,
        name -> Text,
        active -> Bool,
    }
}

diesel::allow_tables_to_appear_in_same_query!(note, user,);
