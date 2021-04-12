use diesel::{
    pg::PgConnection,
    r2d2::{self, ConnectionManager, Pool, PooledConnection as R2d2PooledConnection},
};
pub use crate::error::Error;

pub type Connection = PgConnection;
pub type ConnectionPool = Pool<ConnectionManager<Connection>>;
pub type PooledConnection = R2d2PooledConnection<ConnectionManager<Connection>>;

pub fn new_pool<S: Into<String>>(database_url: S) -> Result<ConnectionPool, Error> {
    let manager = ConnectionManager::<Connection>::new(database_url.into());
    let pool = r2d2::Pool::builder().build(manager)?;
    
    Ok(pool)
}