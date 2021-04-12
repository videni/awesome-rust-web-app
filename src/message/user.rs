use serde::{Deserialize, Serialize};
use validator::{Validate};
use actix::prelude::{Message};
use crate::prelude::Result;
use uuid::Uuid;

#[derive(Debug, Deserialize, Validate)]
pub struct Login {
    #[validate(length(min = 6, max = 30, message = "登录名必须为6-30个字符长"))]
    pub username: String,
    #[validate(length(min = 8, max = 30, message = "密码长度必须为8-30个字符"))]
    pub password: String,
}

impl Message for Login {
    type Result = Result<LoginResponse>;
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub token: String,
}

#[derive(Debug, Validate, Deserialize)]
pub struct Register {
    #[validate(length(min = 6, max = 30, message = "登录名必须为6-30个字符长"))]
    pub username: String,
    #[validate(email(message = "不是合法的邮件地址"))]
    pub email: String,
    #[validate(length(min = 8, max = 30, message = "密码长度必须为8-30个字符"))]
    pub password: String,
}

impl Message for Register {
    type Result = Result<RegisterResponse>;
}

#[derive(Debug, Serialize)]
pub struct RegisterResponse {
    pub token: String,
    pub user_id: Uuid,
    pub email: Option<String>,
    pub username: String,
}

// #[derive(Debug, Insertable)]
// #[table_name = "user"]
// pub struct CreateUser {
//     pub username: String,
//     pub email: String,
//     pub password: String,
// }

// #[derive(Debug, AsChangeset)]
// #[table_name = "user"]
// pub struct EnabeUser {
//     pub user_id: Uuid,
// }

// #[derive(Debug, AsChangeset)]
// #[table_name = "user"]
// pub struct DisableUser {
//     pub user_id: Uuid,
// }

// #[derive(Debug, AsChangeset)]
// #[table_name = "user"]
// pub struct RemoveUser {
//     pub user_id: Uuid,
// }
