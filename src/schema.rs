// @generated automatically by Diesel CLI.

diesel::table! {
    categories (id) {
        id -> Int4,
        name -> Varchar,
    }
}

diesel::table! {
    meditations (id) {
        id -> Int4,
        title -> Varchar,
        description -> Text,
        category_id -> Int4,
        tags -> Varchar,
        script -> Text,
        image_url -> Varchar,
        audio_url -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        email -> Varchar,
        password -> Varchar,
        profile_picture -> Nullable<Varchar>,
        role -> Varchar,
        status -> Varchar,
        plan -> Varchar,
    }
}

diesel::joinable!(meditations -> categories (category_id));

diesel::allow_tables_to_appear_in_same_query!(
    categories,
    meditations,
    users,
);
