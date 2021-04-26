table! {
    user (user_id) {
        user_id -> Uuid,
        username -> Varchar,
        password -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        email -> Nullable<Varchar>,
        failures_num -> Nullable<Int2>,
        first_failed_at -> Nullable<Timestamp>,
        lock_expires_at -> Nullable<Timestamp>,
        enabled -> Bool,
        salt -> Nullable<Varchar>,
    }
}
