use chrono::{NaiveDateTime, Duration, Utc};
use diesel::{Queryable, Identifiable};
use crate::schema::user;
use crate::service::jwt::{CanGenerateJwt, get_secret, Payload};
use jsonwebtoken::{encode, Header, EncodingKey};
use crate::prelude::*;
use uuid::Uuid;

#[derive(Debug, Queryable,Identifiable)]
#[primary_key(user_id)]
#[column_name(user_id)]
#[table_name="user"]
pub struct User {
    pub user_id: Uuid,
    pub username: String,
    pub password: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub email: Option<String>,
    pub failures_num: Option<i16>,
    pub first_failed_at: Option<NaiveDateTime>,
    pub lock_expires_at: Option<NaiveDateTime>,
    pub enabled: bool,
    pub salt: Option<String>,
}

impl CanGenerateJwt for User {
    fn generate_jwt(&self) -> Result<String> {
        let expires_at = (Utc::now() + Duration::days(21)).timestamp();
        let payload = Payload { user_id: self.user_id, expires_at };

        let header = Header::default();
        let secret = &get_secret();
        let key = EncodingKey::from_secret(secret.as_ref());
        let token = encode(&header, &payload, &key)?;

        Ok(token)
    }
}

#[derive(Debug, Insertable)]
#[table_name = "user"]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, AsChangeset)]
#[table_name = "user"]
pub struct UserChange {
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
}
