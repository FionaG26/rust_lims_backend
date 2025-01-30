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
        #[max_length = 255]
        username -> Varchar,
        #[max_length = 255]
        password -> Varchar,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    samples,
    users,
);
