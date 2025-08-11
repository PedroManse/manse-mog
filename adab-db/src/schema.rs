// @generated automatically by Diesel CLI.

diesel::table! {
    entries (rowid) {
        rowid -> Integer,
        song -> Text,
        band -> Text,
        date -> Date,
        deezer_link -> Nullable<Text>,
        spotify_link -> Nullable<Text>,
    }
}
