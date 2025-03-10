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

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        password -> Varchar,
        created_at -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    samples,
    users,
);
