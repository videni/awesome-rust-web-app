use chrono::NaiveDateTime;

#[derive(Debug, Insertable)]
#[table_name = "user"]
pub struct CreateUser {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, AsChangeset)]
#[table_name = "user"]
pub struct EnabeUser {
    pub user_id: u32,
}

#[derive(Debug, AsChangeset)]
#[table_name = "user"]
pub struct DisableUser {
    pub user_id: u32,
}

#[derive(Debug, AsChangeset)]
#[table_name = "user"]
pub struct RemoveUser {
    pub user_id: u32,
}