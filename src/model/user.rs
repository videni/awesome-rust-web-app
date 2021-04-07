use chrono::NaiveDateTime;
use diesel::{Queryable, Identifiable};
use crate::schema::user;

#[derive(Debug, Queryable,Identifiable)]
#[primary_key(user_id)]
#[column_name(user_id)]
#[table_name="user"]
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