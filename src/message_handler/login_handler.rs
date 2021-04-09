use crate::message::user::{Login, LoginResponse};
use crate::message_handler::MessageHandler;
use actix::prelude::Handler;
use crate::prelude::{Result, Error};
use crate::model::user::User as UserModel;
use diesel::prelude::*;
use crate::service::security::encoder::Argon2PasswordEncoder;
use crate::service::security::encoder::PasswordEncoder;
use serde_json::{json};
use crate::service::jwt::CanGenerateJwt;

impl Handler<Login> for MessageHandler {
    type Result = Result<LoginResponse>;

    fn handle(&mut self, msg: Login, _: &mut Self::Context) -> Self::Result {
        use crate::schema::user::{table, columns};

        let raw_password = &msg.password;

        let conn = &self.db_connection_pool.get()?;

        let user: UserModel = table.filter(columns::username.eq(msg.username)).first(conn)?;
        let encoder = Argon2PasswordEncoder {};

        if encoder.is_password_valid(&user.password, raw_password, None) {
            let response = LoginResponse{
                token: user.generate_jwt().unwrap(),
            };
            Ok(response)
        } else {
            Err(Error::Unauthorized(json!({
                "error": "Wrong password",
            })))
        }
    }
}