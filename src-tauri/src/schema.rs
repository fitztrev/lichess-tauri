// @generated automatically by Diesel CLI.

diesel::table! {
    engines (lichess_id) {
        lichess_id -> Text,
        binary_location -> Text,
    }
}

diesel::table! {
    settings (key) {
        key -> Text,
        value -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(engines, settings,);
