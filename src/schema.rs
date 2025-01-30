// @generated automatically by Diesel CLI.

diesel::table! {
    samples (id) {
        id -> Int4,
        name -> Varchar,
        sample_type -> Varchar,
        collected_at -> Timestamp,
        status -> Varchar,
    }
}
