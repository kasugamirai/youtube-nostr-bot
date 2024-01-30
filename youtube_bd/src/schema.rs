// @generated automatically by Diesel CLI.

diesel::table! {
    videos (id) {
        id -> Int4,
        author -> Varchar,
        channel -> Varchar,
        title -> Varchar,
        link -> Varchar,
        published -> Bool,
        userid -> Int4,
    }
}

diesel::table! {
    youtube_users (id) {
        id -> Int4,
        username -> Varchar,
        publickey -> Varchar,
        privatekey -> Varchar,
        channel -> Varchar,
        channel_id -> Varchar,
    }
}

diesel::joinable!(videos -> youtube_users (userid));

diesel::allow_tables_to_appear_in_same_query!(
    videos,
    youtube_users,
);
