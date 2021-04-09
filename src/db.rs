use diesel::{
    mysql::MysqlConnection,
    r2d2::{self, ConnectionManager, Pool, PooledConnection},
};
pub use crate::error::Error;

pub type Connection = MysqlConnection;
pub type ConnectionPool = Pool<ConnectionManager<Connection>>;

pub fn new_pool<S: Into<String>>(database_url: S) -> Result<ConnectionPool, Error> {
    let manager = ConnectionManager::<Connection>::new(database_url.into());
    let pool = r2d2::Pool::builder().build(manager)?;
    Ok(pool)
}