use validator::Validate;
use serde::Deserialize;
// use diesel::{Insertable, AsChangeset};


#[derive(Debug, Deserialize, Validate)]
pub struct LoginUser {
    #[validate(length(
        min = "6",
        max = "30",
        message = "验证失败: 登录名必须为6-30个字符长"
    ))]
    pub username: String,
    #[validate(length(
        min = "8",
        max = "30",
        message = "验证失败：密码长度必须为8-30个字符"
    ))]
    pub password: String,
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
//     pub user_id: u32,
// }

// #[derive(Debug, AsChangeset)]
// #[table_name = "user"]
// pub struct DisableUser {
//     pub user_id: u32,
// }

// #[derive(Debug, AsChangeset)]
// #[table_name = "user"]
// pub struct RemoveUser {
//     pub user_id: u32,
// }
