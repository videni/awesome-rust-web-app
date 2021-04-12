use diesel::table;

table! {
    user (user_id) {
        user_id -> Uuid,
        username -> Text,
        password -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        email -> Nullable<Text>,
        failures_num -> Nullable<Smallint>,
        first_failed_at -> Nullable<Timestamp>,
        lock_expires_at -> Nullable<Timestamp>,
        enabled -> Bool,
        salt -> Nullable<Text>,
    }
}
