// @generated automatically by Diesel CLI.

diesel::table! {
    engines (engine_id) {
        engine_id -> Text,
        binary_location -> Text,
        uci_options -> Text,
    }
}

diesel::table! {
    settings (key) {
        key -> Text,
        value -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(engines, settings,);
