use crate::db::ConnectionPool;
use crate::error::Error;
use crate::message::user::Login;
use crate::message_handler::MessageHandler;
use crate::model::user::User as UserModel;
use crate::service::jwt::CanGenerateJwt;
use crate::service::security::encoder::Argon2PasswordEncoder;
use crate::service::security::encoder::PasswordEncoder;
use async_trait::async_trait;
use diesel::prelude::*;
use serde_json::json;

pub struct LoginHandler(pub ConnectionPool);

#[async_trait]
impl MessageHandler for LoginHandler {
    type Message = Login;
    type Reply = String;
    type Error = Error;

    async fn handle(&self, msg: Login) -> Result<String, Error> {
        use crate::schema::user::{columns, table};

        let raw_password = &msg.password;

        let conn = &self.0.get()?;

        let user: UserModel = table
            .filter(columns::username.eq(msg.username))
            .first(conn)?;
        let encoder = Argon2PasswordEncoder {};

        if encoder.is_password_valid(&user.password, raw_password, None) {
            Ok(user.generate_jwt().unwrap())
        } else {
            Err(Error::Unauthorized(json!({
                "error": "Wrong password",
            })))
        }
    }
}
