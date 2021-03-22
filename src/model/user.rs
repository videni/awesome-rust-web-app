use chrono::NaiveDateTime;

use crate::schema::users;

#[derive(Debug, Queryable, Identifiable)]
pub struct User {
    pub user_id: u32,
    pub username: String,
    pub password: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub email: String,
    pub failures_num: u16,
    pub first_failed_at: NaiveDateTime,
    pub lock_expires_at: NaiveDateTime,
    pub enabled: u16,
    pub salt: String,
}