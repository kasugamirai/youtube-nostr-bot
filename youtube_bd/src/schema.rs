// @generated automatically by Diesel CLI.

diesel::table! {
    apk_counters (id) {
        id -> Int8,
        action_date -> Timestamptz,
        user_apk_counter -> Int8,
    }
}

diesel::table! {
    nostr_keys (id) {
        id -> Int8,
        public_key -> Varchar,
        private_key -> Nullable<Varchar>,
    }
}

diesel::table! {
    time_stamps (id) {
        id -> Int8,
        timestamp -> Int8,
        action_date -> Nullable<Timestamptz>,
        user_timestamp -> Int8,
    }
}

diesel::table! {
    user_login_activates (id) {
        id -> Int8,
        action_date -> Timestamptz,
        user_login_activate -> Int8,
    }
}

diesel::table! {
    user_login_data (id) {
        id -> Int8,
        action_date -> Timestamptz,
        user_login -> Int8,
    }
}

diesel::table! {
    user_relays (id) {
        id -> Int8,
        relay -> Varchar,
        npub -> Varchar,
        is_liked -> Bool,
        is_hidden -> Bool,
        action_date -> Timestamptz,
    }
}

diesel::table! {
    user_session_metrics (id) {
        id -> Int8,
        action_count -> Int8,
        crash_count -> Int8,
        error_count -> Int8,
        frustration_count -> Int8,
        time_spent -> Int8,
        action_date -> Nullable<Timestamptz>,
        user_metrics -> Int8,
    }
}

diesel::table! {
    user_sign_up_data (id) {
        id -> Int8,
        action_date -> Timestamptz,
        user_signup -> Int8,
    }
}

diesel::table! {
    users (id) {
        id -> Int8,
        public_key -> Varchar,
        inserted_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    videos (id) {
        id -> Int4,
        author -> Varchar,
        title -> Varchar,
        link -> Varchar,
        published -> Bool,
    }
}

diesel::joinable!(apk_counters -> users (user_apk_counter));
diesel::joinable!(time_stamps -> users (user_timestamp));
diesel::joinable!(user_login_activates -> users (user_login_activate));
diesel::joinable!(user_login_data -> users (user_login));
diesel::joinable!(user_session_metrics -> users (user_metrics));
diesel::joinable!(user_sign_up_data -> users (user_signup));

diesel::allow_tables_to_appear_in_same_query!(
    apk_counters,
    nostr_keys,
    time_stamps,
    user_login_activates,
    user_login_data,
    user_relays,
    user_session_metrics,
    user_sign_up_data,
    users,
    videos,
);
