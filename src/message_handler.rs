use crate::db::ConnectionPool;
use actix::prelude::{Actor, SyncContext};

pub mod user;

pub struct MessageHandler
{
    db_connection_pool: ConnectionPool,
}

impl Actor for MessageHandler {
    type Context = SyncContext<Self>;
}