use crate::message::user::{Login};
use crate::message_handler::MessageHandler;
use actix::prelude::Handler;
use crate::prelude::{Result, Error};
use crate::model::user::*;
use diesel::prelude::*;
use crate::service::security::encoder::Argon2PasswordEncoder;
use crate::service::security::encoder::PasswordEncoder;
use serde_json::{json};
use crate::schema::user::dsl::user as users;

impl Handler<Login> for MessageHandler {
    type Result = Result<()>;

    fn handle(&mut self, msg: Login, _: &mut Self::Context) -> Self::Result {
        let raw_password = &msg.password;

        let conn = &self.db_connection_pool.get()?;

        let user: User = users.filter(email.eq(msg.email)).first(conn)?;
        let encoder = Argon2PasswordEncoder {};

        if encoder.is_password_valid(user.password, raw_password) {
            Ok(user.into())
        } else {
            Err(Error::Unauthorized(json!({
                "error": "Wrong password",
            })))
        }
    }
}