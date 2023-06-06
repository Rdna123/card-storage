// @generated automatically by Diesel CLI.

diesel::table! {
    card (card_name) {
        card_name -> Nullable<Text>,
        amount -> Nullable<Integer>,
        card_set -> Text,
        card_formats -> Text,
    }
}

diesel::table! {
    mtg_formats (format_name) {
        format_name -> Nullable<Text>,
    }
}

diesel::table! {
    mtg_sets (code) {
        code -> Nullable<Text>,
    }
}

diesel::table! {
    posts (id) {
        id -> Nullable<Integer>,
        title -> Text,
        body -> Text,
        published -> Bool,
    }
}

diesel::joinable!(card -> mtg_formats (card_formats));
diesel::joinable!(card -> mtg_sets (card_set));

diesel::allow_tables_to_appear_in_same_query!(
    card,
    mtg_formats,
    mtg_sets,
    posts,
);
