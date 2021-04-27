use serde::{Deserialize};
use validator::{Validate};
use uuid::Uuid;


#[derive(Debug, Deserialize, Validate)]
pub struct Login {
    #[validate(length(min = 6, max = 30, message = "Username must be 6-30 charaters long"))]
    pub username: String,
    #[validate(length(min = 8, max = 30, message = "Password must be 8-30 chracters long"))]
    pub password: String,
}

#[derive(Debug)]
pub struct CreateUser {
    pub username: String,
    pub email: Option<String>,
    pub password: String,
    pub user_id: Uuid,
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
