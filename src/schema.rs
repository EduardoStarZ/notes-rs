// @generated automatically by Diesel CLI.

diesel::table! {
    notes (id) {
        id -> Integer,
        name -> Text,
        content -> Text,
        user_id -> Integer
    }
}

diesel::table! {
    users (id) {
        id -> Integer,
        name -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    notes,
    users,
);
