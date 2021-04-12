use crate::message::user::{Login, LoginResponse, Register, RegisterResponse};
use crate::message_handler::MessageHandler;
use actix::prelude::Handler;
use crate::prelude::{Result, Error};
use crate::model::user::{User as UserModel, NewUser};
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

impl Handler<Register> for MessageHandler {
    type Result = Result<RegisterResponse>;

    fn handle(&mut self, msg: Register, _: &mut Self::Context) -> Self::Result {
        use crate::schema::user::{table};

        let encoder = Argon2PasswordEncoder {};

        let new_user = NewUser {
            username: msg.username.clone(),
            email: msg.email.clone(),
            password: encoder.encode_password(&msg.password, None),
        };

        let conn = &self.db_connection_pool.get()?;

        match diesel::insert_into(table)
            .values(new_user)
            .get_result::<UserModel>(conn)
        {
            Ok(user) => Ok(RegisterResponse {
                token: user.generate_jwt().unwrap(),
                username: user.username,
                user_id: user.user_id,
                email: user.email,
            }),
            Err(e) => Err(e.into()),
        }
    }
}