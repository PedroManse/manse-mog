// @generated automatically by Diesel CLI.

diesel::table! {
    entries (rowid) {
        rowid -> Integer,
        source_id -> Integer,
        song -> Text,
        band -> Text,
        date -> Date,
        link -> Nullable<Text>,
        album_250px -> Nullable<Text>,
        album_56px -> Nullable<Text>,
    }
}

diesel::table! {
    sources (source_id) {
        source_id -> Nullable<Integer>,
        site -> Text,
    }
}

diesel::joinable!(entries -> sources (source_id));

diesel::allow_tables_to_appear_in_same_query!(
    entries,
    sources,
);
