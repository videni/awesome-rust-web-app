use chrono::{NaiveDateTime, Duration, Utc};
use diesel::{Queryable, Identifiable};
use crate::schema::user;
use crate::service::jwt::{CanGenerateJwt, get_secret, Payload};
use jsonwebtoken::{encode, Header, EncodingKey};
use crate::prelude::*;

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
    pub email: Option<String>,
    pub failures_num: Option<i8>,
    pub first_failed_at: Option<NaiveDateTime>,
    pub lock_expires_at: Option<NaiveDateTime>,
    pub enabled: i8,
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
